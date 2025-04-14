//! # ```axum_folder_router``` Macro Documentation
//!
//! [folder_router!] is a procedural macro for the Axum web framework that automatically generates router boilerplate based on your file structure. It simplifies route organization by using filesystem conventions to define your API routes.
//!
//! ## Installation
//!
//! Add the dependency to your ```Cargo.toml```:
//!
//! ```toml
//! [dependencies]
//! axum_folder_router = "0.1.0"
//! axum = "0.7"
//! ```
//!
//! ## Basic Usage
//!
//! The macro scans a directory for ```route.rs``` files and automatically creates an Axum router based on the file structure:
//!
//! ```rust,no_run
#![doc = include_str!("../examples/simple/main.rs")]
//! ```
//!
//! ## File Structure Convention
//!
//! The macro converts your file structure into routes:
//!
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
//!
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
//!
//! ```text
//! src/api/users/[id]/route.rs   -> "/users/{id}"
//! ```
//!
//! Inside the route handler:
//!
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
//!
//! ```text
//! src/api/files/[...path]/route.rs   -> "/files/*path"
//! ```
//!
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
//!
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
//!

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::fs;
use std::path::{Path, PathBuf};
use syn::{Ident, LitStr, Result, Token, parse::Parse, parse::ParseStream, parse_macro_input};

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

/// Creates an Axum router by scanning a directory for `route.rs` files.
///
/// # Parameters
///
/// * `path` - A string literal pointing to the API directory, relative to the Cargo manifest directory
/// * `state_type` - The type name of your application state that will be shared across all routes
///
/// # Example
///
/// ```rust
/// # use axum_folder_router::folder_router;
/// # #[derive(Debug, Clone)]
/// # struct AppState ();
/// #
/// let router = folder_router!("./src/api", AppState);
/// ```
///
/// This will scan all `route.rs` files in the `./src/api` directory and its subdirectories,
/// automatically mapping their path structure to URL routes with the specified state type.
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

    // Generate module definitions and route registrations
    let mut module_defs = Vec::new();
    let mut route_registrations = Vec::new();

    for (route_path, rel_path) in routes {
        // Generate module name and axum path
        let (axum_path, mod_name) = path_to_route_info(&rel_path);
        let mod_ident = format_ident!("{}", mod_name);

        // Create module path for include!
        let rel_file_path = route_path.strip_prefix(&manifest_dir).unwrap();
        let rel_file_str = rel_file_path.to_string_lossy().to_string();

        // Add module definition
        module_defs.push(quote! {
            #[allow(warnings)]
            mod #mod_ident {
                include!(concat!(env!("CARGO_MANIFEST_DIR"), "/", #rel_file_str));
            }
        });

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

            let mut builder = quote! {
                axum::routing::#first_method_ident(#mod_ident::#first_method_ident)
            };

            for (_method, method_ident) in &method_registrations[1..] {
                builder = quote! {
                    #builder.#method_ident(#mod_ident::#method_ident)
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
        {
            #(#module_defs)*

            let mut router = axum::Router::<#state_type>::new();
            #(#route_registrations)*
            router
        }
    };

    expanded.into()
}

// Recursively collect route.rs files
fn collect_route_files(base_dir: &Path, current_dir: &Path, routes: &mut Vec<(PathBuf, PathBuf)>) {
    if let Ok(entries) = fs::read_dir(current_dir) {
        for entry in entries.filter_map(std::result::Result::ok) {
            let path = entry.path();

            if path.is_dir() {
                collect_route_files(base_dir, &path, routes);
            } else if path.file_name().unwrap_or_default() == "route.rs" {
                if let Some(parent) = path.parent() {
                    if let Ok(rel_dir) = parent.strip_prefix(base_dir) {
                        routes.push((path.clone(), rel_dir.to_path_buf()));
                    }
                }
            }
        }
    }
}

// Convert a relative path to (axum_path, mod_name)
fn path_to_route_info(rel_path: &Path) -> (String, String) {
    if rel_path.components().count() == 0 {
        return ("/".to_string(), "root".to_string());
    }

    let mut axum_path = String::new();
    let mut mod_name = String::new();

    for segment in rel_path.iter() {
        let s = segment.to_str().unwrap_or_default();
        if s.starts_with('[') && s.ends_with(']') {
            let inner = &s[1..s.len() - 1];
            if let Some(param) = inner.strip_prefix("...") {
                axum_path.push_str(&format!("/*{}", param));
                mod_name.push_str(&format!("__{}", param));
            } else {
                axum_path.push_str(&format!("/{{{}}}", inner));
                mod_name.push_str(&format!("__{}", inner));
            }
        } else {
            axum_path.push('/');
            axum_path.push_str(s);
            mod_name.push_str("__");
            mod_name.push_str(s);
        }
    }

    (axum_path, mod_name.trim_start_matches('_').to_string())
}
