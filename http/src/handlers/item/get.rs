use axum::{http::StatusCode, response::IntoResponse};

pub(crate) async fn get_item() -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}
