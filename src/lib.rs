/*!

[`macro@folder_router`] is a procedural macro for the Axum web framework
that automatically generates router boilerplate based on your file
structure. It simplifies route organization by using filesystem conventions
to define your API routes.

# Installation

Add the dependency to your ```Cargo.toml```:

```toml
[dependencies]
axum_folder_router = "0.3"
axum = "0.8"
```

See [Avoiding Cache Issues](#avoiding-cache-issues) on how to fix cargos
caching, which may cause new ```route.rs``` files to be ignored.

# Crate Features

* **nightly** -
  Enables use of unstable [`track_path`](https://doc.rust-lang.org/beta/unstable-book/library-features/track-path.html) feature to [avoid cache issues](#avoiding-cache-issues).
* **debug** -
  Adds some debug logging

# Basic Usage

The macro scans a directory for ```route.rs``` files and automatically
creates an Axum router based on the file structure:

```rust,no_run
*/
#![doc = include_str!("../examples/simple/main.rs")]
/*!
```

## Folder Structure

The macro converts your file structure into routes:
```text
src/api/
├── route.rs                 -> "/"
├── hello/
│   └── route.rs             -> "/hello"
├── users/
│   ├── route.rs             -> "/users"
│   └── [id]/
│       └── route.rs         -> "/users/{id}"
└── files/
    └── [...path]/
        └── route.rs         -> "/files/\*path"
```

Each ```route.rs``` file can contain HTTP method handlers that are automatically mapped to the corresponding route.

## Route Handlers

Inside each ```route.rs``` file, define async functions named after HTTP methods:
```rust
*/
#![doc = include_str!("../examples/simple/api/route.rs")]
/*!
```

# Detailed Usage

## HTTP Methods

The macro supports all standard HTTP methods as defined in RFC9110.
- ```get```
- ```post```
- ```put```
- ```delete```
- ```patch```
- ```head```
- ```options```
- ```trace```
- ```connect```

And additionally
- ```any```, which matches all methods

## Path Parameters

Dynamic path segments are defined using brackets:
```text
src/api/users/[id]/route.rs   -> "/users/{id}"
```

Inside the route handler:
```rust
use axum::{
  extract::Path,
  response::IntoResponse
};

pub async fn get(Path(id): Path<String>) -> impl IntoResponse {
    format!("User ID: {}", id)
}
```

## Catch-all Parameters

Use the spread syntax for catch-all segments:
```text
src/api/files/[...path]/route.rs   -> "/files/\*path"
```
```rust
use axum::{
  extract::Path,
  response::IntoResponse
};

pub async fn get(Path(path): Path<String>) -> impl IntoResponse {
    format!("Requested file path: {}", path)
}
```

## State Extraction

The state type provided to the macro is available in all route handlers:
All routes share the same state type, though you can use ```FromRef``` for more granular state extraction.
```rust
use axum::{
  extract::State,
  response::IntoResponse
};

# #[derive(Debug, Clone)]
# struct AppState ();

pub async fn get(State(state): State<AppState>) -> impl IntoResponse {
    format!("State: {:?}", state)
}
```

## Avoiding Cache Issues

By default newly created route.rs files may be ignored due to cargo's build-in caching.

### Nightly Rust

If you're using a nightly toolchain, just enable the `nightly` feature.
```toml
[dependencies]
axum_folder_router = { version = "0.3", features = ["nightly"] }
```
This enables us to use the unstable [`track_path`](https://doc.rust-lang.org/beta/unstable-book/library-features/track-path.html) API to tell cargo to watch for changes in your route directories.

### Stable Rust (requires `build.rs`)

On stable, you'll need to add this `build.rs` to your project root:
```rust
fn main() {
   // Watch routes folder, so it picks up new routes
   println!(
       "cargo:rerun-if-changed={routes_folder}",
       routes_folder = "my/routes" // Replace with your actual routes dir
   );
}
```
*/
#![forbid(unsafe_code)]
#![cfg_attr(feature = "nightly", feature(track_path))]

#[cfg(feature = "nightly")]
use proc_macro::tracked_path;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse_macro_input;

mod generate;
mod parse;
#[cfg(feature="debug")]
mod debug;

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

    let mut errors = TokenStream2::new();

    let args = parse_macro_input!(attr as parse::FolderRouterArgs);

    #[cfg(feature = "nightly")]
    {
        #[cfg(feature = "debug")]
        println!(
            "/// [folder_router] Tracking path: {:?}",
            args.abs_norm_path()
        );
        tracked_path::path(args.abs_norm_path().as_path().to_str().unwrap());
    }

    let item = parse_macro_input!(item as parse::FolderRouterItem);
    let routes = parse::FolderRouterRoutes::parse_from_path(&mut errors, &args.abs_norm_path());

    let module_tree = generate::module_tree(&args, &item, &routes);
    let router_impl = generate::router_impl(&mut errors, &args, &item, &routes);

    quote! {
      #item
      #errors
      #module_tree
      #router_impl
    }
    .into()
}
