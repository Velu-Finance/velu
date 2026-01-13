use crate::AppState;
use axum::Router;
use std::sync::Arc;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

pub mod health;
pub mod root;

pub fn create_router(state: Arc<AppState>) -> Router {
    let api_routes = Router::new()
        .merge(health::routes());

    Router::new()
        .merge(root::routes())
        .nest("/api", api_routes)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state)
}