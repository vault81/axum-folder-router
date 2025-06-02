//! # ```axum_folder_router``` Macro Documentation
//!
//! [macro@folder_router] is a procedural macro for the Axum web framework that
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
//! axum_folder_router = "0.3"
//! axum = "0.8"
//! ```
//!
//! See [Avoiding Cache Issues](#avoiding-cache-issues) on how to fix cargos
//! caching, which may cause new ```route.rs``` files to be ignored.
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
//! The macro supports all standard HTTP methods as defined in RFC9110.
//! - ```get```
//! - ```post```
//! - ```put```
//! - ```delete```
//! - ```patch```
//! - ```head```
//! - ```options```
//! - ```trace```
//! - ```connect```
//!
//! And additionally
//! - ```any```, which matches all methods
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
//! - **Expects separate directory**: To make rust-analyzer & co work correctly the macro imports all route.rs files inside the given directory tree.
//!   It is highly recommended to keep the route directory separate from the rest of your module-tree.
//! - **Cargo's build-in caching may ignore newly created route.rs files**: See [Avoiding Cache Issues](#avoiding-cache-issues)
//!
//! ### Avoiding Cache Issues
//!
//! By default newly created route.rs files may be ignored due to cargo's build-in caching.
//!
//! #### Nightly Rust
//!
//! If you're using a nightly toolchain, just enable the `nightly` feature.
//! ```toml
//! [dependencies]
//! axum_folder_router = { version = "0.3", features = ["nightly"] }
//! ```
//!
//! This enables us to use the unstable [`track_path`](https://doc.rust-lang.org/beta/unstable-book/library-features/track-path.html) API to tell cargo to watch for changes in your route directories.
//!
//! #### Stable Rust (requires `build.rs`)
//!
//! On stable, you'll need to add this `build.rs` to your project root:
//! ```rust
//! fn main() {
//!    // Watch routes folder, so it picks up new routes
//!    println!(
//!        "cargo:rerun-if-changed={routes_folder}",
//!        routes_folder = "my/routes" // Replace with your actual routes dir
//!    );
//! }
//! ```
//!

#![cfg_attr(feature = "nightly", feature(track_path))]

use std::path::Path;

#[cfg(feature = "nightly")]
use proc_macro::tracked_path;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

mod generate;
mod parse;

/// Creates an Axum router module tree & creation function
/// by scanning a directory for `route.rs` files.
///
/// # Parameters
///
/// * `path` - A string literal pointing to the route directory, relative to the
///   Cargo manifest directory
/// * `state_type` - The type name of your application state that will be shared
///   across all routes
#[allow(clippy::missing_panics_doc)]
#[proc_macro_attribute]
pub fn folder_router(attr: TokenStream, item: TokenStream) -> TokenStream {
    #[cfg(feature = "debug")]
    println!(
        "/// [folder_router] Running folder_router macro attrs:({}) item: {}",
        attr, item
    );

    let args = parse_macro_input!(attr as parse::FolderRouterArgs);
    let input_item = parse_macro_input!(item as syn::ItemStruct);
    let struct_name = &input_item.ident;

    let base_path = args.path;
    let state_type = args.state_type;

    let manifest_dir = get_manifest_dir();
    let base_dir = Path::new(&manifest_dir).join(&base_path);

    #[cfg(feature = "nightly")]
    {
        #[cfg(feature = "debug")]
        println!("/// [folder_router] Tracking path: {:?}", base_dir);
        tracked_path::path(base_dir.to_str().unwrap());
    }

    let mod_namespace = format!(
        "__folder_router__{}__{}",
        struct_name
            .to_string()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '_' })
            .map(|c| c.to_ascii_lowercase())
            .collect::<String>(),
        base_path
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '_' })
            .collect::<String>()
    );

    let routes = parse::collect_route_files(&base_dir, &base_dir);

    if routes.is_empty() {
        return TokenStream::from(quote! {
            compile_error!(concat!("No route.rs files found in the specified directory: '",
                #base_path,
                "'. Make sure the path is correct and contains route.rs files."
            ));
        });
    }

    let module_tree = generate::module_tree(&mod_namespace, &base_dir, &routes);
    let route_registrations = generate::route_registrations(&mod_namespace, &routes);

    quote! {
      #module_tree

      #input_item

      impl #struct_name {
          pub fn into_router() -> axum::Router<#state_type> {
              let mut router = axum::Router::new();
              #route_registrations
              router
          }
      }
    }
    .into()
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
