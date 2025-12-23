/// [folder_router] Running folder_router macro attrs:("examples/advanced/api", AppState) item: struct MyFolderRouter();
/// [folder_router] Tracking path: "/home/tristand/code/axum-folder-router/examples/advanced/api"
/// [folder_router] Found route.rs for axum_path: "/files/{*path}", mod_path: ["files", "___path", "route"]
/// [folder_router] Found methods for axum_path: "/files/{*path}", mod_path: ["files", "___path", "route"], methods: ["get"]
/// [folder_router] Found route.rs for axum_path: "/files", mod_path: ["files", "route"]
/// [folder_router] Found methods for axum_path: "/files", mod_path: ["files", "route"], methods: ["get", "post"]
/// [folder_router] Found route.rs for axum_path: "/ping", mod_path: ["ping", "route"]
/// [folder_router] Found methods for axum_path: "/ping", mod_path: ["ping", "route"], methods: ["any", "get"]
/// [folder_router] Found route.rs for axum_path: "/", mod_path: ["route"]
/// [folder_router] Found methods for axum_path: "/", mod_path: ["route"], methods: ["get", "post"]
/// [folder_router] Found route.rs for axum_path: "/users/{:id}", mod_path: ["users", "__id", "route"]
/// [folder_router] Found methods for axum_path: "/users/{:id}", mod_path: ["users", "__id", "route"], methods: ["get"]
/// [folder_router] Found route.rs for axum_path: "/users", mod_path: ["users", "route"]
/// [folder_router] Found methods for axum_path: "/users", mod_path: ["users", "route"], methods: ["get", "post"]
#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2021::*;
use axum_folder_router::folder_router;
struct AppState {
    _foo: String,
}
#[automatically_derived]
impl ::core::clone::Clone for AppState {
    #[inline]
    fn clone(&self) -> AppState {
        AppState {
            _foo: ::core::clone::Clone::clone(&self._foo),
        }
    }
}
struct MyFolderRouter();
#[path = "/home/tristand/code/axum-folder-router/examples/advanced/api"]
mod __folder_router__myfolderrouter {
    #[path = "route.rs"]
    pub mod route {
        use axum::response::{Html, IntoResponse};
        pub async fn get() -> impl IntoResponse {
            Html("<h1>Hello World!</h1>").into_response()
        }
        pub async fn post() -> impl IntoResponse {
            "Posted successfully".into_response()
        }
    }
    #[path = "files"]
    pub mod files {
        #[path = "route.rs"]
        pub mod route {
            use axum::response::{Html, IntoResponse};
            pub async fn get() -> impl IntoResponse {
                Html("<h1>Hello World!</h1>").into_response()
            }
            pub async fn post() -> impl IntoResponse {
                "Posted successfully".into_response()
            }
        }
        #[path = "[...path]"]
        pub mod ___path {
            #[path = "route.rs"]
            pub mod route {
                use axum::{extract::Path, response::IntoResponse};
                pub async fn get(Path(path): Path<String>) -> impl IntoResponse {
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("Requested file path: {0}", path),
                        )
                    })
                }
            }
        }
    }
    #[path = "ping"]
    pub mod ping {
        #[path = "route.rs"]
        pub mod route {
            use axum::response::Html;
            use axum::response::IntoResponse;
            pub async fn get() -> impl IntoResponse {
                Html("<h1>GET Pong!</h1>").into_response()
            }
            pub async fn any() -> impl IntoResponse {
                Html("<h1>ANY Pong!</h1>").into_response()
            }
        }
    }
    #[path = "users"]
    pub mod users {
        #[path = "route.rs"]
        pub mod route {
            use axum::response::{Html, IntoResponse};
            pub async fn get() -> impl IntoResponse {
                Html("<h1>Hello World!</h1>").into_response()
            }
            pub async fn post() -> impl IntoResponse {
                "Posted successfully".into_response()
            }
        }
        #[path = "[id]"]
        pub mod __id {
            #[path = "route.rs"]
            pub mod route {
                use axum::{extract::Path, response::IntoResponse};
                pub async fn get(Path(id): Path<String>) -> impl IntoResponse {
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("User ID: {0}", id))
                    })
                }
            }
        }
    }
}
impl MyFolderRouter {
    pub fn into_router() -> axum::Router<AppState> {
        let mut router = axum::Router::new();
        router = router
            .route(
                "/files/{*path}",
                axum::routing::get(
                    __folder_router__myfolderrouter::files::___path::route::get,
                ),
            );
        router = router
            .route(
                "/files",
                axum::routing::get(__folder_router__myfolderrouter::files::route::get)
                    .post(__folder_router__myfolderrouter::files::route::post),
            );
        router = router
            .route(
                "/ping",
                axum::routing::any(__folder_router__myfolderrouter::ping::route::any)
                    .get(__folder_router__myfolderrouter::ping::route::get),
            );
        router = router
            .route(
                "/",
                axum::routing::get(__folder_router__myfolderrouter::route::get)
                    .post(__folder_router__myfolderrouter::route::post),
            );
        router = router
            .route(
                "/users/{:id}",
                axum::routing::get(
                    __folder_router__myfolderrouter::users::__id::route::get,
                ),
            );
        router = router
            .route(
                "/users",
                axum::routing::get(__folder_router__myfolderrouter::users::route::get)
                    .post(__folder_router__myfolderrouter::users::route::post),
            );
        router
    }
}
