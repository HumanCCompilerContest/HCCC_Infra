use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "web_server=debug");
    }
    tracing_subscriber::fmt::init();

    web_server::setup_session_store().await;

    let app = web_server::app().await;

    let addr = SocketAddr::from(([127, 0, 0, 1], 55301));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
