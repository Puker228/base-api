use axum::{Json, Router, routing::get};
use serde::Serialize;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[derive(Serialize)]
struct MessageResponse {
    message: &'static str,
}

async fn hello() -> Json<MessageResponse> {
    info!("GET /hello");
    Json(MessageResponse {
        message: "привет"
    })
}

async fn bye() -> Json<MessageResponse> {
    info!("GET /bye");
    Json(MessageResponse {
        message: "пока"
    })
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse().unwrap()))
        .init();

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/hello", get(hello))
        .route("/bye", get(bye))
        .layer(TraceLayer::new_for_http());

    info!("listening on 0.0.0.0:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
