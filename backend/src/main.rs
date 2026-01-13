use velu_backend::create_app;

#[tokio::main]
async fn main() {
    let app = create_app().await.expect("Failed to create app");

    app.run().await;
}
