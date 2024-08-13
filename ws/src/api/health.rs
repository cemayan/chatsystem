use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::json;

pub fn create_route() -> Router {
    Router::new()
        .route("/", get(health_check))
}

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({ "message": "ws is operational"})))
}