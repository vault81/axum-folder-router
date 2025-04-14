# Unreleased

## Breaking
### Rework into attribute macro.

Instead of using it like this

```rust
// ...
folder_router!("./examples/simple/api", AppState);
// ...
let folder_router: Router<AppState> = folder_router();
```

It now works like this:
```rust
// ...
#[folder_router("./examples/simple/api", AppState)]
struct MyFolderRouter
// ...
let folder_router: Router<AppState> = MyFolderRouter::into_router();
```

This is a bit cleaner & it allows you to have multiple separate folder-based Routers.

# 0.2.3
- Refactored the detection of which methods exist,
  we actually parse the file now instead of just checking that it contains `pub async #method_name`

# 0.2.2
- Re-licensed to MIT

# 0.2.1
- Documentation & test improvements

# 0.2.0
- Generate module imports instead of `include!`ing, so rust-analyzer works.

# 0.1.0
- MVP adapted from https://github.com/richardanaya/axum-folder-router-htmx
