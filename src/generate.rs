use std::{collections::BTreeMap, fmt::Write, path::Path};

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::LitStr;

use crate::parse::{self, methods_for_route};

// A struct representing a directory in the module tree
#[derive(Debug)]
struct ModuleDir {
    name: String,
    has_route: bool,
    children: BTreeMap<String, ModuleDir>,
}

impl ModuleDir {
    fn new(name: &str) -> Self {
        ModuleDir {
            name: name.to_string(),
            has_route: false,
            children: BTreeMap::new(),
        }
    }

    fn add_to_module_tree(&mut self, rel_path: &Path, _route_path: &Path) {
        let components: Vec<_> = rel_path
            .components()
            .map(|c| c.as_os_str().to_string_lossy().to_string())
            .collect();

        if components.is_empty() {
            self.has_route = true;
            return;
        }

        let mut root = self;

        for (i, segment) in components.iter().enumerate() {
            if i == components.len() - 1 && segment == "route.rs" {
                root.has_route = true;
                break;
            }

            root = root
                .children
                .entry(segment.clone())
                .or_insert_with(|| ModuleDir::new(segment));
        }
    }
}

// Add a route to the module tree

// Normalize a path segment for use as a module name
fn normalize_module_name(name: &str) -> String {
    if name.starts_with('[') && name.ends_with(']') {
        let inner = &name[1..name.len() - 1];
        if let Some(stripped) = inner.strip_prefix("...") {
            format!("___{stripped}")
        } else {
            format!("__{inner}")
        }
    } else {
        name.replace(['-', '.'], "_")
    }
}

// Convert a relative path to module path segments and axum route path
fn path_to_module_path(rel_path: &Path) -> (String, Vec<String>) {
    let mut axum_path = String::new();
    let mut mod_path = Vec::new();

    let components: Vec<_> = rel_path
        .components()
        .map(|c| c.as_os_str().to_string_lossy().to_string())
        .collect();

    // Handle root route
    if components.is_empty() {
        return ("/".to_string(), vec!["route".to_string()]);
    }

    for (i, segment) in components.iter().enumerate() {
        if i == components.len() - 1 && segment == "route.rs" {
            mod_path.push("route".to_string());
        } else {
            // Process directory name
            let normalized = normalize_module_name(segment);
            mod_path.push(normalized);

            // Process URL path
            if segment.starts_with('[') && segment.ends_with(']') {
                let param = &segment[1..segment.len() - 1];
                if let Some(stripped) = param.strip_prefix("...") {
                    write!(&mut axum_path, "/{{*{stripped}}}").unwrap();
                } else {
                    write!(&mut axum_path, "/{{:{param}}}").unwrap();
                }
            } else {
                write!(&mut axum_path, "/{segment}").unwrap();
            }
        }
    }

    if axum_path.is_empty() {
        axum_path = "/".to_string();
    }

    (axum_path, mod_path)
}

// Generate tokens for a module path
fn generate_mod_path_tokens(mod_path: &[String]) -> TokenStream {
    let mut result = TokenStream::new();

    for (i, segment) in mod_path.iter().enumerate() {
        let segment_ident = format_ident!("{}", segment);

        if i == 0 {
            result = quote! { #segment_ident };
        } else {
            result = quote! { #result::#segment_ident };
        }
    }

    result
}

// Generate module hierarchy code
fn generate_module_hierarchy(dir: &ModuleDir) -> TokenStream {
    let mut result = TokenStream::new();

    // Add route.rs module if this directory has one
    if dir.has_route {
        let route_mod = quote! {
            #[path = "route.rs"]
            pub mod route;
        };
        result.extend(route_mod);
    }

    // Add subdirectories
    for child in dir.children.values() {
        let child_name = format_ident!("{}", normalize_module_name(&child.name));
        let child_path_lit = LitStr::new(&child.name, proc_macro2::Span::call_site());
        let child_content = generate_module_hierarchy(child);

        let child_mod = quote! {
            #[path = #child_path_lit]
            pub mod #child_name {
                #child_content
            }
        };

        result.extend(child_mod);
    }

    result
}

fn route_registrations(
    errors: &mut TokenStream,
    mod_namespace: &syn::Path,
    routes: &parse::FolderRouterRoutes,
) -> TokenStream {
    let mut route_method_registrations = Vec::new();
    for (route_path, rel_path) in routes {
        // Generate module path and axum path
        let (axum_path, mod_path) = path_to_module_path(&rel_path);

        #[cfg(feature = "debug")]
        println!(
            "/// [folder_router] Found route.rs for axum_path: {:?}, mod_path: {:?}",
            axum_path, mod_path
        );

        let method_registrations = methods_for_route(&route_path);

        #[cfg(feature = "debug")]
        println!(
            "/// [folder_router] Found methods for axum_path: {:?}, mod_path: {:?}, methods: {:?}",
            axum_path, mod_path, method_registrations
        );

        if !method_registrations.is_empty() {
            let first_method = &method_registrations[0];
            let first_method_ident = format_ident!("{}", first_method);

            let mod_path_tokens = generate_mod_path_tokens(&mod_path);

            let mut builder = quote! {
                axum::routing::#first_method_ident(#mod_namespace::#mod_path_tokens::#first_method_ident)
            };

            for method in &method_registrations[1..] {
                let method_ident = format_ident!("{}", method);

                builder = quote! {
                    #builder.#method_ident(#mod_namespace::#mod_path_tokens::#method_ident)
                };
            }

            let registration = quote! {
                router = router.route(#axum_path, #builder);
            };
            route_method_registrations.push(registration);
        }
    }
    if route_method_registrations.is_empty() {
        errors.extend(quote! {
            compile_error!(concat!(
                "No routes defined in your route.rs's !\n",
                "Ensure that at least one `pub async fn` named after an HTTP verb is defined. (e.g. get, post, put, delete)"
            ));
        });
    }

    TokenStream::from_iter(route_method_registrations)
}

pub fn router_impl(
    errors: &mut TokenStream,
    args: &parse::FolderRouterArgs,
    item: &parse::FolderRouterItem,
    routes: &parse::FolderRouterRoutes,
) -> TokenStream {
    let struct_name = item.struct_name();
    let state_type = args.state_type.clone();
    let registrations = route_registrations(errors, &item.module_namespace(), routes);

    quote! {
        impl #struct_name {
            pub fn into_router() -> axum::Router<#state_type> {
                let mut router = axum::Router::new();
                #registrations
                router
            }
        }
    }
}

pub fn module_tree(
    args: &parse::FolderRouterArgs,
    item: &parse::FolderRouterItem,
    routes: &parse::FolderRouterRoutes,
) -> TokenStream {
    let base_path_lit = LitStr::new(
        args.abs_norm_path().as_path().to_str().unwrap(),
        proc_macro2::Span::call_site(),
    );

    let mod_namespace = item.module_namespace();

    let mod_str = mod_namespace.to_token_stream().to_string();
    let mut root = ModuleDir::new(&mod_str);
    for (route_path, rel_path) in routes {
        root.add_to_module_tree(&rel_path, &route_path);
    }

    let mod_hierarchy = generate_module_hierarchy(&root);
    quote! {
        #[path = #base_path_lit]
        mod #mod_namespace {
            #mod_hierarchy
        }
    }
}
