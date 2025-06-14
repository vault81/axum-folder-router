# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

- Nothing yet

## [0.3.9] - 2025-06-04

- Refactored the internals a bit
- Reworded some docs

## [0.3.8] - 2025-06-02

- Add `nightly` feature to fix caching issues without build.rs via [`track_path`](https://doc.rust-lang.org/beta/unstable-book/library-features/track-path.html) unstable API.
- Add `debug` feature for adding some logging

## [0.3.7] - 2025-06-02

- Add docs for rerun-if-changed in build.rs so new routes are picked up when compiling with cache (Thanks to @imbolc)

## [0.3.6] - 2025-04-17

- Better error messages when having route.rs files with invalid code

## [0.3.5] - 2025-04-16

- Moved macrotest to dev deps

## [0.3.4] - 2025-04-16

- Refactored huge lib.rs into 3 seperate files.
- Downgraded edition to 2021 for better compatability

## [0.3.3] - 2025-04-15

### Added
- Add support for remaining HTTP methods
  - we no support the full set as defined by rfc9110
  - trace & connect were missing specifically
- Add support for `any` axum router method (default method router, others will take precedence)

## [0.3.2] - 2025-04-15
- Refactor internals
- Add solid testing
  - explicitly test generated macro output using macrotest
  - test error output using trybuilt

## [0.3.1] - 2025-04-15

- Fix invalid doc links

## [0.3.0] - 2025-04-15

After some experimentation, the API has begun to stabilize. This should likely be the last breaking change for some time.

### Breaking Changes

- **Reworked implementation into an attribute macro**
  - Previous implementation required function calls:
    ```rust
    folder_router!("./examples/simple/api", AppState);
    // ...
    let folder_router: Router<AppState> = folder_router();
    ```
  - New implementation uses an attribute macro:
    ```rust
    #[folder_router("./examples/simple/api", AppState)]
    struct MyFolderRouter;
    // ...
    let folder_router: Router<AppState> = MyFolderRouter::into_router();
    ```
  - This approach provides a cleaner API and allows for multiple separate folder-based Routers

## [0.2.3] - 2025-04-14

### Changed
- **Improved method detection** - Now properly parses files instead of using string matching
  - Previous version checked if file contained ```pub async #method_name```
  - New version properly parses the file using `syn` for more accurate detection

## [0.2.2] - 2025-04-14

### Changed
- **License changed to MIT**

## [0.2.1] - 2025-04-14

### Improved
- Enhanced documentation
- Added more comprehensive tests

## [0.2.0] - 2024-04-14

### Changed
- **Improved code integration** 
  - Generate module imports instead of using ```include!```
  - Makes the code compatible with rust-analyzer
  - Provides better IDE support

## [0.1.0] - 2024-04-14

### Added
- Initial release
- Minimum viable product adapted from https://github.com/richardanaya/axum-folder-router-htmx
