use std::{fmt::Write, path::Path};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::LitStr;

use crate::parse::{self, methods_for_route};

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
        return ("/".to_string(), vec![]);
    }

    for (i, segment) in components.iter().enumerate() {
        if i == components.len() - 1 {
            continue;
        }
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
            result = quote! { ::#segment_ident };
        } else {
            result = quote! { ::#result::#segment_ident };
        }
    }

    crate::dbg(format_args!("{result}"));
    result
}

fn route_registrations(
    _errors: &mut TokenStream,
    mod_namespace: &syn::Path,
    routes: &parse::FolderRouterRoutes,
) -> TokenStream {
    let mut route_method_registrations = Vec::new();
    for (route_path, rel_path) in routes {
        // Generate module path and axum path
        let (axum_path, mod_path) = path_to_module_path(&rel_path);

        crate::dbg(format_args!(
            "Found file: {}, path: {axum_path:?}, mod_path: {mod_path:?}",
            rel_path.display()
        ));

        let method_registrations = methods_for_route(&route_path);

        if !method_registrations.is_empty() {
            crate::dbg(format_args!(
                "Found methods for file: {}, methods: {method_registrations:?}",
                rel_path.display()
            ));

            let first_method = format_ident!("{}", method_registrations[0]);

            let mod_path_tokens = generate_mod_path_tokens(&mod_path);

            let mut method_router = quote! {
                axum::routing::#first_method(__folder_router::#mod_namespace #mod_path_tokens::#first_method)
            };

            for method in &method_registrations[1..] {
                let next_method = format_ident!("{}", method);

                method_router = quote! {
                    #method_router.#next_method(__folder_router::#mod_namespace #mod_path_tokens::#next_method)
                };
            }

            let registration = quote! {
                router = router.route(#axum_path, #method_router);
            };
            route_method_registrations.push(registration);
        }
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
    if registrations.is_empty() {
        errors.extend(quote::quote! {
            compile_error!(concat!(
                "No methods defined in your files!\n",
                "Ensure that at least one `pub async fn` named after an HTTP verb is defined. (e.g. get, post, put, delete)"
            ));
        });
    }

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

pub fn module_tree(args: &parse::FolderRouterArgs, item: &parse::FolderRouterItem) -> TokenStream {
    let base_path_lit = LitStr::new(
        args.abs_norm_path().as_path().to_str().unwrap(),
        proc_macro2::Span::call_site(),
    );

    let mod_namespace = item.module_namespace();

    quote! {
        pub(crate) mod __folder_router {
            #[path = #base_path_lit]
            pub(crate) mod #mod_namespace;
        }
    }
}
