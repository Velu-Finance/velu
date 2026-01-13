use crate::handlers::root;
use crate::AppState;
use axum::{routing::get, Router};
use std::sync::Arc;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/", get(root::root))
}
