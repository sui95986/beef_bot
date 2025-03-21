use crate::handlers::handle_chat_message;
use crate::handlers::handle_welcome_message;
use serde_json::Value;
use tungstenite::Message;

pub async fn decide(msg: Message) {
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
