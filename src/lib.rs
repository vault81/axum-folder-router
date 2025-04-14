//! # ```axum_folder_router``` Macro Documentation
//!
//! [folder_router!] is a procedural macro for the Axum web framework that
//! automatically generates router boilerplate based on your file structure. It
//! simplifies route organization by using filesystem conventions to define your
//! API routes.
//!
//! ## Installation
//!
//! Add the dependency to your ```Cargo.toml```:
//!
//! ```toml
//! [dependencies]
//! axum_folder_router = "0.1.0"
//! axum = "0.8"
//! ```
//!
//! ## Basic Usage
//!
//! The macro scans a directory for ```route.rs``` files and automatically
//! creates an Axum router based on the file structure:
//!
//! ```rust,no_run
#![doc = include_str!("../examples/simple/main.rs")]
//! ```
//! 
//! ## File Structure Convention
//!
//! The macro converts your file structure into routes:
//! ```text
//! src/api/
//! ├── route.rs                 -> "/"
//! ├── hello/
//! │   └── route.rs             -> "/hello"
//! ├── users/
//! │   ├── route.rs             -> "/users"
//! │   └── [id]/
//! │       └── route.rs         -> "/users/{id}"
//! └── files/
//!     └── [...path]/
//!         └── route.rs         -> "/files/*path"
//! ```
//! 
//! Each ```route.rs``` file can contain HTTP method handlers that are automatically mapped to the corresponding route.
//!
//! ## Route Handlers
//!
//! Inside each ```route.rs``` file, define async functions named after HTTP methods:
//! ```rust
#![doc = include_str!("../examples/simple/api/route.rs")]
//! ```
//! 
//! ## Supported Features
//!
//! ### HTTP Methods
//!
//! The macro supports all standard HTTP methods:
//! - ```get```
//! - ```post```
//! - ```put```
//! - ```delete```
//! - ```patch```
//! - ```head```
//! - ```options```
//!
//! ### Path Parameters
//!
//! Dynamic path segments are defined using brackets:
//! ```text
//! src/api/users/[id]/route.rs   -> "/users/{id}"
//! ```
//! 
//! Inside the route handler:
//! ```rust
//! use axum::{
//!   extract::Path,
//!   response::IntoResponse
//! };
//!
//! pub async fn get(Path(id): Path<String>) -> impl IntoResponse {
//!     format!("User ID: {}", id)
//! }
//! ```
//! 
//! ### Catch-all Parameters
//!
//! Use the spread syntax for catch-all segments:
//! ```text
//! src/api/files/[...path]/route.rs   -> "/files/*path"
//! ```
//! ```rust
//! use axum::{
//!   extract::Path,
//!   response::IntoResponse
//! };
//!
//! pub async fn get(Path(path): Path<String>) -> impl IntoResponse {
//!     format!("Requested file path: {}", path)
//! }
//! ```
//! 
//! ### State Extraction
//!
//! The state type provided to the macro is available in all route handlers:
//! All routes share the same state type, though you can use ```FromRef``` for more granular state extraction.
//! ```rust
//! use axum::{
//!   extract::State,
//!   response::IntoResponse
//! };
//!
//! # #[derive(Debug, Clone)]
//! # struct AppState ();
//!
//! pub async fn get(State(state): State<AppState>) -> impl IntoResponse {
//!     format!("State: {:?}", state)
//! }
//! ```
//! 
//! ## Limitations
//!
//! - **Compile-time Only**: The routing is determined at compile time, so dynamic route registration isn't supported.
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Ident,
    LitStr,
    Result,
    Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

struct FolderRouterArgs {
    path: String,
    state_type: Ident,
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

// A struct representing a directory in the module tree
#[derive(Debug)]
struct ModuleDir {
    name: String,
    has_route: bool,
    children: HashMap<String, ModuleDir>,
}

impl ModuleDir {
    fn new(name: &str) -> Self {
        ModuleDir {
            name: name.to_string(),
            has_route: false,
            children: HashMap::new(),
        }
    }
}

/// Creates an Axum router module tree & creation function
/// by scanning a directory for `route.rs` files.
///
/// # Parameters
///
/// * `path` - A string literal pointing to the API directory, relative to the
///   Cargo manifest directory
/// * `state_type` - The type name of your application state that will be shared
///   across all routes
///
/// # Example
///
/// ```rust
/// use axum_folder_router::folder_router;
/// # #[derive(Debug, Clone)]
/// # struct AppState ();
/// #
/// folder_router!("./src/api", AppState);
/// #
/// fn main() {
///   let router = folder_router();
/// }
/// ```
///
/// This will scan all `route.rs` files in the `./src/api` directory and its
/// subdirectories, automatically mapping their path structure to URL routes
/// with the specified state type.
#[proc_macro]
pub fn folder_router(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as FolderRouterArgs);
    let base_path = args.path;
    let state_type = args.state_type;

    // Get the project root directory
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let base_dir = Path::new(&manifest_dir).join(&base_path);

    // Collect route files
    let mut routes = Vec::new();
    collect_route_files(&base_dir, &base_dir, &mut routes);

    // Build module tree
    let mut root = ModuleDir::new("__folder_router");
    for (route_path, rel_path) in &routes {
        add_to_module_tree(&mut root, rel_path, route_path);
    }

    // Generate module tree
    let root_mod_ident = format_ident!("{}", root.name);

    let base_path_lit = LitStr::new(base_dir.to_str().unwrap(), proc_macro2::Span::call_site());
    let mod_hierarchy = generate_module_hierarchy(&root);

    // Generate route registrations
    let mut route_registrations = Vec::new();
    for (route_path, rel_path) in routes {
        // Generate module path and axum path
        let (axum_path, mod_path) = path_to_module_path(&rel_path);

        // Read the file content to find HTTP methods
        let file_content = fs::read_to_string(&route_path).unwrap_or_default();
        let methods = ["get", "post", "put", "delete", "patch", "head", "options"];

        let mut method_registrations = Vec::new();
        for method in &methods {
            if file_content.contains(&format!("pub async fn {}(", method)) {
                let method_ident = format_ident!("{}", method);
                method_registrations.push((method, method_ident));
            }
        }

        if !method_registrations.is_empty() {
            let (_first_method, first_method_ident) = &method_registrations[0];
            let mod_path_tokens = generate_mod_path_tokens(&mod_path);

            let mut builder = quote! {
                axum::routing::#first_method_ident(#root_mod_ident::#mod_path_tokens::#first_method_ident)
            };

            for (_method, method_ident) in &method_registrations[1..] {
                builder = quote! {
                    #builder.#method_ident(#root_mod_ident::#mod_path_tokens::#method_ident)
                };
            }

            let registration = quote! {
                router = router.route(#axum_path, #builder);
            };
            route_registrations.push(registration);
        }
    }

    // Generate the final code
    let expanded = quote! {
            #[path = #base_path_lit]
            mod #root_mod_ident {
                #mod_hierarchy
            }

            fn folder_router() -> axum::Router::<#state_type> {
                let mut router = axum::Router::<#state_type>::new();
                #(#route_registrations)*
                router
            }
    };

    expanded.into()
}

// Add a path to the module tree
fn add_to_module_tree(root: &mut ModuleDir, rel_path: &Path, _route_path: &Path) {
    let mut current = root;

    let components: Vec<_> = rel_path
        .components()
        .map(|c| c.as_os_str().to_string_lossy().to_string())
        .collect();

    // Handle special case for root route.rs
    if components.is_empty() {
        current.has_route = true;
        return;
    }

    for (i, component) in components.iter().enumerate() {
        // For the file itself (route.rs), we just mark the directory as having a route
        if i == components.len() - 1 && component == "route.rs" {
            current.has_route = true;
            break;
        }

        // For directories, add them to the tree
        let dir_name = component.clone();
        if !current.children.contains_key(&dir_name) {
            current
                .children
                .insert(dir_name.clone(), ModuleDir::new(&dir_name));
        }

        current = current.children.get_mut(&dir_name).unwrap();
    }
}

// Generate module hierarchy code
fn generate_module_hierarchy(dir: &ModuleDir) -> proc_macro2::TokenStream {
    let mut result = proc_macro2::TokenStream::new();

    // panic!("{:?}", dir);
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

// Generate tokens for a module path
fn generate_mod_path_tokens(mod_path: &[String]) -> proc_macro2::TokenStream {
    let mut result = proc_macro2::TokenStream::new();

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

// Normalize a path segment for use as a module name
fn normalize_module_name(name: &str) -> String {
    if name.starts_with('[') && name.ends_with(']') {
        let inner = &name[1..name.len() - 1];
        if let Some(stripped) = inner.strip_prefix("...") {
            format!("___{}", stripped)
        } else {
            format!("__{}", inner)
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
        } else if segment.starts_with('[') && segment.ends_with(']') {
            let inner = &segment[1..segment.len() - 1];
            if let Some(param) = inner.strip_prefix("...") {
                axum_path.push_str(&format!("/{{*{}}}", param));
                mod_path.push(format!("___{}", param));
            } else {
                axum_path.push_str(&format!("/{{{}}}", inner));
                mod_path.push(format!("__{}", inner));
            }
        } else if segment != "route.rs" {
            // Skip the actual route.rs file
            axum_path.push('/');
            axum_path.push_str(segment);
            mod_path.push(normalize_module_name(segment));
        } else {
            println!("blub");
        }
    }

    if axum_path.is_empty() {
        axum_path = "/".to_string();
    }

    (axum_path, mod_path)
}

// Recursively collect route.rs files (unchanged from your original)
fn collect_route_files(base_dir: &Path, current_dir: &Path, routes: &mut Vec<(PathBuf, PathBuf)>) {
    if let Ok(entries) = fs::read_dir(current_dir) {
        for entry in entries.filter_map(std::result::Result::ok) {
            let path = entry.path();

            if path.is_dir() {
                collect_route_files(base_dir, &path, routes);
            } else if path.file_name().unwrap_or_default() == "route.rs" {
                if let Ok(rel_dir) = path.strip_prefix(base_dir) {
                    routes.push((path.clone(), rel_dir.to_path_buf()));
                }

                // if let Some(parent) = path.parent() {
                //     if let Ok(rel_dir) = parent.strip_prefix(base_dir) {
                //         routes.push((path.clone(), rel_dir.to_path_buf()));
                //     }
                // }
            }
        }
    }
}
