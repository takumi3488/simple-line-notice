use axum::{response::IntoResponse, routing::post, Router};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let app = Router::new().route("/", post(handle_broadcast_message));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handle_broadcast_message() -> impl IntoResponse {}
