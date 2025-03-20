mod handlers;

use crate::handlers::handle_welcome_message;
use futures_util::{SinkExt, StreamExt, future, pin_mut};
use serde_json::Value;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::connect_async;
use tungstenite::{Message, client::IntoClientRequest};

#[tokio::main]
async fn main() {
    let url = "wss://eventsub.wss.twitch.tv/ws?keepalive_timeout_seconds=120"
        .into_client_request()
        .unwrap();

    let (ws_stream, _) = connect_async(url).await.expect("Faled to connect");
    println!("Websocket handshake has been successfully completed");

    let (mut write, mut read) = ws_stream.split();

    let read_future = read.for_each(|message| async {
        match message {
            Ok(msg) => {
                // let data = msg.into_data();
                let data_str = msg.to_text().expect("This works?");
                println!("Received:");
                let json: Value = serde_json::from_str(&data_str).unwrap_or(Value::Null);

                if json != Value::Null {
                    let message_type = &json["metadata"]["message_type"];
                    println!("{}", json);
                    if message_type == &Value::String("session_welcome".to_string()) {
                        println!("received welcome message!");
                        handle_welcome_message(&json);
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
