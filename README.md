
[![Crates.io](https://img.shields.io/crates/v/axum-folder-router)](https://crates.io/crates/axum-folder-router)
[![Documentation](https://docs.rs/axum-folder-router/badge.svg)](https://docs.rs/axum-folder-router)
![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

# axum-folder-router

```#[folder_router(...)]``` is a procedural attribute macro for the Axum web framework that automatically generates router boilerplate based on your directory & file structure. 
Inspired by popular frameworks like next.js.

## Features

- **File System-Based Routing**: Define your API routes using intuitive folder structures
- **Reduced Boilerplate**: Automatically generates route mapping code
- **IDE Support**: Generates proper module imports for better rust-analyzer integration
- **Multiple Routers**: Create separate folder-based routers in the same application

## Usage

For detailed instructions see [the examples](./examples) or [docs.rs](https://docs.rs/axum-folder-router).

## License

This repository, is licensed permissively under the terms of the MIT license.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, shall be licensed as above, without any additional terms or conditions.

### Attribution

This macro is based on the [build.rs template by @richardanaya](https://github.com/richardanaya/axum-folder-router-htmx)
