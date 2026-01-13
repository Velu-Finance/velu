use crate::AppState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use std::sync::Arc;

pub async fn health(State(_): State<Arc<AppState>>) -> impl IntoResponse {
    Json(json!({
        "status": "healthy",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

pub async fn ready(State(_): State<Arc<AppState>>) -> StatusCode {
    StatusCode::OK
}
