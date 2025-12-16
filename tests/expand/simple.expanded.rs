/// [folder_router] Running folder_router macro attrs:("examples/simple/routes.rs", AppState) item: struct MyFolderRouter();
/// [folder_router] Tracking path: "/home/tristand/code/axum-folder-router/examples/simple/routes.rs"
/// [folder_router] Collecting route files base_dir: /home/tristand/code/axum-folder-router/examples/simple, dir: /home/tristand/code/axum-folder-router/examples/simple, routes: [("/home/tristand/code/axum-folder-router/examples/simple/main.rs", "main.rs"), ("/home/tristand/code/axum-folder-router/examples/simple/routes.rs", "routes.rs")]
/// [folder_router] Found file: main.rs, path: "/", mod_path: []
/// [folder_router] Found file: routes.rs, path: "/", mod_path: []
/// [folder_router] Found methods for file: routes.rs, methods: ["get"]
/// [folder_router] 
#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2021::*;
use axum_folder_router::folder_router;
struct AppState;
#[automatically_derived]
impl ::core::clone::Clone for AppState {
    #[inline]
    fn clone(&self) -> AppState {
        AppState
    }
}
struct MyFolderRouter();
pub(crate) mod __folder_router {
    #[path = "/home/tristand/code/axum-folder-router/examples/simple/routes.rs"]
    pub(crate) mod myfolderrouter {
        use axum::response::{Html, IntoResponse};
        pub async fn get() -> impl IntoResponse {
            Html("<h1>Hello World!</h1>").into_response()
        }
    }
}
impl MyFolderRouter {
    pub fn into_router() -> axum::Router<AppState> {
        let mut router = axum::Router::new();
        router = router
            .route("/", axum::routing::get(__folder_router::myfolderrouter::get));
        router
    }
}
