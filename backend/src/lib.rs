use anyhow::Result;
use axum::Router;
use std::net::SocketAddr;
use std::sync::Arc;

pub mod config;
pub mod handlers;
pub mod routes;
pub mod telemetry;

#[derive(Clone)]
pub struct AppState {
    pub config: config::Config,
}

pub struct App {
    pub router: Router,
    pub addr: SocketAddr,
}

pub async fn create_app() -> Result<App> {
    let config = config::Config::from_env()?;
    telemetry::init();

    let addr = config.addr();
    let state = Arc::new(AppState {
        config: config.clone(),
    });

    let router = routes::create_router(state);

    Ok(App { router, addr })
}

impl App {
    pub async fn run(self) {
        let listener = tokio::net::TcpListener::bind(self.addr)
            .await
            .expect("Failed to bind");

        tracing::info!("Server on {}", self.addr);

        axum::serve(listener, self.router)
            .with_graceful_shutdown(shutdown_signal())
            .await
            .unwrap();
    }
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c().await.ok();
}
