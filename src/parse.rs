use std::{
    fs,
    path::{Path, PathBuf},
};

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
