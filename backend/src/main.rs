use anyhow::{Context, Result};
use velu_backend::create_app;

#[tokio::main]
async fn main() -> Result<()> {
    let app = create_app().await.context("Failed to create app")?;

    app.run().await.context("Failed to run app")?;

    Ok(())
}
