[![Crates.io](https://img.shields.io/crates/v/axum-folder-router.svg)](https://crates.io/crates/axum-folder-router)
[![Workflow Status](https://github.com/vault81/axum-folder-router/workflows/main/badge.svg)](https://github.com/vault81/axum-folder-router/actions?query=workflow%3A%22main%22)
![Maintenance](https://img.shields.io/badge/maintenance-activly--developed-brightgreen.svg)

# axum_folder_router

## ```axum_folder_router``` Macro Documentation

```folder_router``` is a procedural macro for the Axum web framework that automatically generates router configurations based on your file structure. It simplifies route organization by using filesystem conventions to define your API routes.

### Installation

Add the dependency to your ```Cargo.toml```:

```toml
[dependencies]
axum_folder_router = "0.1.0"
axum = "0.7"
```

### Basic Usage

The macro scans a directory for ```route.rs``` files and automatically creates an Axum router based on the file structure:

```rust

## License

This repository, like all my personal projects, is licensed under the GNU Affero General Public License v3.0 or later (AGPL-3.0-or-later). 
This ensures that modifications to the code remain open source when used in network services. 
Contact me if this doesn't suit your needs.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the GNU Affero General Public License v3.0 or later (AGPL-3.0-or-later) license, shall be licensed as above, without any additional terms or conditions.
