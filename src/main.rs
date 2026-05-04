use axum::{Json, Router, routing::get};
use serde::Serialize;

#[derive(Serialize)]
struct MessageResponse {
    message: &'static str,
}

async fn hello() -> Json<MessageResponse> {
    Json(MessageResponse {
        message: "привет"
    })
}

async fn bye() -> Json<MessageResponse> {
    Json(MessageResponse {
        message: "пока"
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/hello", get(hello))
        .route("/bye", get(bye));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
