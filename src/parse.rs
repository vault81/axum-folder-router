use std::{
    fs,
    path::{Path, PathBuf},
};

use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    parse_file,
    Ident,
    Item,
    LitStr,
    Result,
    Token,
    Visibility,
};

#[derive(Debug)]
pub struct FolderRouterArgs {
    pub path: String,
    pub state_type: Ident,
}

impl FolderRouterArgs {
    pub fn abs_norm_path(&self) -> PathBuf {
        let base_path = self.path.clone();

        let manifest_dir = Self::get_manifest_dir();
        let base_dir = Path::new(&manifest_dir).join(&base_path);

        base_dir
    }

    // This is a workaround for macrotest behaviour
    #[cfg(debug_assertions)]
    fn get_manifest_dir() -> String {
        use regex::Regex;
        let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or("./".to_string());
        let re = Regex::new(r"^(.+)/target/tests/axum-folder-router/[A-Za-z0-9]{42}$").unwrap();

        if let Some(captures) = re.captures(&dir) {
            captures.get(1).unwrap().as_str().to_string()
        } else {
            dir
        }
    }

    #[cfg(not(debug_assertions))]
    fn get_manifest_dir() -> String {
        std::env::var("CARGO_MANIFEST_DIR").unwrap_or("./".to_string())
    }
}
impl Parse for FolderRouterArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let path_lit = input.parse::<LitStr>()?;
        input.parse::<Token![,]>()?;
        let state_type = input.parse::<Ident>()?;

        Ok(FolderRouterArgs {
            path: path_lit.value(),
            state_type,
        })
    }
}

/// Parses the file at the specified location and returns HTTP verb functions
pub fn methods_for_route(route_path: &PathBuf) -> Vec<&'static str> {
    // Read the file content
    let Ok(file_content) = fs::read_to_string(route_path) else {
        return Vec::new();
    };

    // Parse the file content into a syn syntax tree
    let Ok(file) = parse_file(&file_content) else {
        return Vec::new();
    };

    // Define HTTP methods we're looking for
    let allowed_methods = [
        "any", "get", "post", "put", "delete", "patch", "head", "options", "trace", "connect",
    ];
    let mut found_methods = Vec::new();

    // Collect all pub & async fn's
    for item in &file.items {
        if let Item::Fn(fn_item) = item {
            let fn_name = fn_item.sig.ident.to_string();
            let is_public = matches!(fn_item.vis, Visibility::Public(_));
            let is_async = fn_item.sig.asyncness.is_some();

            if is_public && is_async {
                found_methods.push(fn_name);
            }
        }
    }

    // Iterate through methods to ensure consistent order
    allowed_methods
        .into_iter()
        .filter(|elem| {
            found_methods
                .clone()
                .into_iter()
                .any(|method| method == *elem)
        })
        .collect()
}

// Collect route.rs files recursively
pub fn collect_route_files(base_dir: &Path, dir: &Path) -> Vec<(PathBuf, PathBuf)> {
    let mut routes = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(std::result::Result::ok) {
            let path = entry.path();

            if path.is_dir() {
                let mut nested_routes = collect_route_files(base_dir, &path);
                routes.append(&mut nested_routes);
            } else if path.file_name().unwrap_or_default() == "route.rs" {
                if let Ok(rel_dir) = path.strip_prefix(base_dir) {
                    routes.push((path.clone(), rel_dir.to_path_buf()));
                }
            }
        }
    }
    routes.sort();
    routes
}

pub struct FolderRouterItem {
    item: syn::ItemStruct,
}

impl FolderRouterItem {
    pub fn module_namespace(&self) -> syn::Path {
        syn::parse_str(&format!(
            "__folder_router__{}",
            self.item
                .ident
                .to_string()
                .chars()
                .map(|c| if c.is_alphanumeric() { c } else { '_' })
                .map(|c| c.to_ascii_lowercase())
                .collect::<String>(),
        ))
        .unwrap()
    }

    pub fn struct_name(&self) -> syn::Ident {
        self.item.ident.clone()
    }
}

impl Parse for FolderRouterItem {
    fn parse(input: ParseStream) -> Result<Self> {
        let item: syn::ItemStruct = input.parse()?;

        Ok(Self {
            item,
        })
    }
}

impl ToTokens for FolderRouterItem {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.item.to_tokens(tokens);
    }
}

pub struct FolderRouterRoutes {
    routes: Vec<(PathBuf, PathBuf)>,
}

impl FolderRouterRoutes {
    pub fn parse_from_path(errors: &mut proc_macro2::TokenStream, path: &Path) -> Self {
        let routes = collect_route_files(path, path);
        let path = path.to_str().unwrap();

        if routes.is_empty() {
            errors.extend(quote::quote! {
                compile_error!(concat!("No route.rs files found in the specified directory: '",
                    #path,
                    "'. Make sure the path is correct and contains route.rs files."
                ));
            });
        }

        Self {
            routes,
        }
    }
}

impl IntoIterator for &FolderRouterRoutes {
    type IntoIter = std::vec::IntoIter<Self::Item>;
    type Item = (std::path::PathBuf, std::path::PathBuf);

    fn into_iter(self) -> Self::IntoIter {
        self.routes.clone().into_iter()
    }
}
