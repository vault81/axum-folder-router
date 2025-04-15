use axum::response::Html;
use axum::response::IntoResponse;

pub async fn get() -> impl IntoResponse {
    Html("<h1>GET Pong!</h1>").into_response()
}

// This tests that our macro generates the routes in the correct order
// as any is only allowable as a first route.
pub async fn any() -> impl IntoResponse {
    Html("<h1>ANY Pong!</h1>").into_response()
}
