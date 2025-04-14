use axum::{extract::Path, response::IntoResponse};

pub async fn get(Path(id): Path<String>) -> impl IntoResponse {
    format!("User ID: {}", id)
}
