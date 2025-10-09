use std::{collections::HashMap, env};

use axum::{Router, response::IntoResponse, routing::get};
use serde::Deserialize;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let app = Router::new().route("/", get(health_check).post(handle_broadcast_message));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> impl IntoResponse {
    "OK"
}

#[derive(Deserialize)]
struct JsonBody {
    text: String,
}

enum NotificationService {
    Line,
    Oshirase,
}

impl NotificationService {
    fn from_env() -> Self {
        match env::var("NOTIFICATION_SERVICE")
            .unwrap_or_else(|_| "line".to_string())
            .to_lowercase()
            .as_str()
        {
            "oshirase" => Self::Oshirase,
            _ => Self::Line,
        }
    }
}

async fn handle_broadcast_message(body: String) -> impl IntoResponse {
    let message_text = match serde_json::from_str::<JsonBody>(&body) {
        Ok(json) => json.text,
        Err(_) => body,
    };

    let token = env::var("LINE_TOKEN").expect("LINE_TOKEN is not set");
    println!("TOKEN: {}", token);

    let service = NotificationService::from_env();
    let client = reqwest::Client::new();

    let res = match service {
        NotificationService::Line => {
            let mut map = HashMap::new();
            let mut message_map = HashMap::new();
            message_map.insert("type", "text");
            message_map.insert("text", &message_text);
            map.insert("messages", vec![message_map]);

            client
                .post("https://api.line.me/v2/bot/message/broadcast")
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", token))
                .json(&map)
                .send()
                .await
        }
        NotificationService::Oshirase => {
            let mut params = HashMap::new();
            params.insert("message", message_text.as_str());

            client
                .post("https://oshirase.app/api/notify")
                .header("Authorization", format!("Bearer {}", token))
                .form(&params)
                .send()
                .await
        }
    };

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
