//! This crate is the web server for contest site of HCCC.
//! Fronend: [Human-C-Compiler-Contest_frontend](https://github.com/HumanCCompilerContest/Human-C-Compiler-Contest_frontend)
use std::net::SocketAddr;

/// Main function
#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "web_server=debug");
    }
    tracing_subscriber::fmt::init();

    web_server::setup_session_store().await;

    let app = web_server::app().await;

    let addr = SocketAddr::from(([0, 0, 0, 0], 55301));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
