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
#[path = "/home/tristand/code/axum-folder-router/examples/advanced/api"]
mod __folder_router__myfolderrouter__examples_advanced_api {
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
                        let res = ::alloc::fmt::format(
                            format_args!("Requested file path: {0}", path),
                        );
                        res
                    })
                }
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
                        let res = ::alloc::fmt::format(format_args!("User ID: {0}", id));
                        res
                    })
                }
            }
        }
    }
}
struct MyFolderRouter();
impl MyFolderRouter {
    pub fn into_router() -> axum::Router<AppState> {
        let mut router = axum::Router::new();
        router = router
            .route(
                "/files/{*path}",
                axum::routing::get(
                    __folder_router__myfolderrouter__examples_advanced_api::files::___path::route::get,
                ),
            );
        router = router
            .route(
                "/files",
                axum::routing::get(
                        __folder_router__myfolderrouter__examples_advanced_api::files::route::get,
                    )
                    .post(
                        __folder_router__myfolderrouter__examples_advanced_api::files::route::post,
                    ),
            );
        router = router
            .route(
                "/",
                axum::routing::get(
                        __folder_router__myfolderrouter__examples_advanced_api::route::get,
                    )
                    .post(
                        __folder_router__myfolderrouter__examples_advanced_api::route::post,
                    ),
            );
        router = router
            .route(
                "/users/{:id}",
                axum::routing::get(
                    __folder_router__myfolderrouter__examples_advanced_api::users::__id::route::get,
                ),
            );
        router = router
            .route(
                "/users",
                axum::routing::get(
                        __folder_router__myfolderrouter__examples_advanced_api::users::route::get,
                    )
                    .post(
                        __folder_router__myfolderrouter__examples_advanced_api::users::route::post,
                    ),
            );
        router
    }
}
