/// [folder_router] Running folder_router macro attrs:("examples/simple/api", AppState) item: struct MyFolderRouter();
/// [folder_router] Tracking path: "/home/tristand/code/axum-folder-router/examples/simple/api"
/// [folder_router] Found route.rs for axum_path: "/", mod_path: ["route"]
/// [folder_router] Found methods for axum_path: "/", mod_path: ["route"], methods: ["get"]
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use axum_folder_router::folder_router;
struct AppState;
#[automatically_derived]
impl ::core::clone::Clone for AppState {
    #[inline]
    fn clone(&self) -> AppState {
        AppState
    }
}
#[path = "/home/tristand/code/axum-folder-router/examples/simple/api"]
mod __folder_router__myfolderrouter__examples_simple_api {
    #[path = "route.rs"]
    pub mod route {
        use axum::response::{Html, IntoResponse};
        pub async fn get() -> impl IntoResponse {
            Html("<h1>Hello World!</h1>").into_response()
        }
    }
}
struct MyFolderRouter();
impl MyFolderRouter {
    pub fn into_router() -> axum::Router<AppState> {
        let mut router = axum::Router::new();
        router = router
            .route(
                "/",
                axum::routing::get(
                    __folder_router__myfolderrouter__examples_simple_api::route::get,
                ),
            );
        router
    }
}
