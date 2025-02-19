use axum::{routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/liveness", get(liveness))
        .route("/readiness", get(readiness));

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn liveness() -> &'static str {
    "ok"
}

async fn readiness() -> &'static str {
    "ok"
}
