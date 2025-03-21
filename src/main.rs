mod handlers;
mod setup;

use crate::handlers::handle_welcome_message;
use dotenv::dotenv;
use futures_util::StreamExt;
use handlers::handle_chat_message;
use serde_json::Value;
use setup::validate_oauth_token;
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::connect_async;
use tungstenite::client::IntoClientRequest;

#[tokio::main]
async fn main() {
    dotenv().ok(); // Sets up vars from .env file
    validate_oauth_token().await;

    let url = "wss://eventsub.wss.twitch.tv/ws?keepalive_timeout_seconds=120"
        .into_client_request()
        .unwrap();

    let (ws_stream, _) = connect_async(url).await.expect("Faled to connect");
    println!("Websocket handshake has been successfully completed");

    let (mut _write, read) = ws_stream.split();

    let read_future = read.for_each(|message| async {
        match message {
            Ok(msg) => {
                let data_str = msg.to_text().expect("Fatal error parsing message to text");
                let json: Value = serde_json::from_str(data_str).unwrap_or(Value::Null);

                if json != Value::Null {
                    let message_type = &json["metadata"]["message_type"];
                    if message_type == &Value::String("session_welcome".to_string()) {
                        println!("received welcome message!");
                        handle_welcome_message(&json).await;
                    } else if message_type == "notification" {
                        handle_chat_message(&json).await;
                    }
                }
            }
            Err(e) => {
                eprintln!("WebSocket error: {}", e);
            }
        }
    });
    read_future.await;
}
