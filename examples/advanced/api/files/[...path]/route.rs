use axum::{extract::Path, response::IntoResponse};

pub async fn get(Path(path): Path<String>) -> impl IntoResponse {
    format!("Requested file path: {}", path)
}
