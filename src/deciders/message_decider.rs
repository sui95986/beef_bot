use crate::handlers::NotificationHandler;
use crate::handlers::handle_unknown_message;
use crate::handlers::handle_welcome_message;
use serde_json::Value;
use tokio_tungstenite::tungstenite::Message;

pub struct MessageDecider {
    notification_handler: NotificationHandler,
}

impl MessageDecider {
    pub fn new(notification_handler: NotificationHandler) -> MessageDecider {
        MessageDecider {
            notification_handler,
        }
    }

    pub async fn decide(&self, msg: Message) {
        let data_str = msg.to_text().expect("Fatal error parsing message to text");

        let json: Value = serde_json::from_str(data_str).unwrap_or(Value::Null);

        if json != Value::Null {
            let message_type = &json["metadata"]["message_type"];
            if message_type == &Value::String("session_welcome".to_string()) {
                println!("received welcome message!");
                handle_welcome_message(&json).await;
            } else if message_type == "notification" {
                let subscription_type = &json["metadata"]["subscription_type"];
                if subscription_type == "channel.chat.message" {
                    self.notification_handler.handle_chat_message(&json).await;
                } else if subscription_type == "channel.ad_break.begin" {
                    self.notification_handler.handle_ad_break_begin(&json).await;
                } else {
                    println!("Unknown notification type: {}", subscription_type);
                }
            } else {
                handle_unknown_message(&json).await;
            }
        } else if !msg.to_string().is_empty() {
            println!("Failed to parse incoming message into json: {}", msg);
        }
    }
}
