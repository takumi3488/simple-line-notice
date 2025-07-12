use std::{collections::HashMap, env};

use axum::{Router, response::IntoResponse, routing::get};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let app = Router::new().route("/", get(health_check).post(handle_broadcast_message));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> impl IntoResponse {
    "OK"
}

async fn handle_broadcast_message(body: String) -> impl IntoResponse {
    let token = env::var("LINE_TOKEN").expect("LINE_TOKEN is not set");
    println!("TOKEN: {}", token);
    let client = reqwest::Client::new();
    let mut map = HashMap::new();
    let mut message_map = HashMap::new();
    message_map.insert("type", "text");
    message_map.insert("text", &body);
    map.insert("messages", vec![message_map]);
    let res = client
        .post("https://api.line.me/v2/bot/message/broadcast")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .json(&map)
        .send()
        .await;
    match res {
        Ok(res) => {
            if res.status().is_success() {
                "Success"
            } else {
                eprintln!("Error: {:?}", res);
                "Error"
            }
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
            "Error"
        }
    }
}
