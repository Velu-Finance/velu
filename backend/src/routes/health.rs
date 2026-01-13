use crate::{handlers::health, AppState};
use axum::{routing::get, Router};
use std::sync::Arc;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/health", get(health::health))
        .route("/health/ready", get(health::ready))
}
