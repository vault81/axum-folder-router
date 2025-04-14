use axum::response::{Html, IntoResponse};

pub async fn get() -> impl IntoResponse {
    Html("<h1>Hello World!</h1>").into_response()
}
