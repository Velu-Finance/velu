use axum::{response::IntoResponse, Json};
use serde_json::json;

pub async fn root() -> impl IntoResponse {
    Json(json!({
        "name": "Velu API",
        "version": env!("CARGO_PKG_VERSION"),
        "status": "ok"
    }))
}
