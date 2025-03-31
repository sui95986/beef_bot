use serde_json::Value;

use crate::{brain::Brain, clients::TwitchApiClient};
use dotenv_codegen::dotenv;

pub struct NotificationHandler {
    twitch_api_client: TwitchApiClient,
    brain: Brain,
}

impl NotificationHandler {
    pub fn new(twitch_api_client: TwitchApiClient, brain: Brain) -> NotificationHandler {
        NotificationHandler {
            twitch_api_client,
            brain,
        }
    }

    pub async fn handle_chat_message(&self, json: &Value) {
        let bot_user_id = dotenv!("BOT_USER_ID");
        let event = &json["payload"]["event"];
        let chatter = event["chatter_user_name"].as_str().unwrap();
        let chatter_user_id = event["chatter_user_id"].as_str().unwrap();
        let message = event["message"]["text"].as_str().unwrap();
        println!("{}: {}", chatter, message);
        if chatter_user_id != bot_user_id {
            if message.starts_with("!") {
                let string_message = String::from(message);
                let mut parts = string_message.splitn(2, " ");
                let cmd = parts.next().unwrap_or("");
                let message = parts.next().unwrap_or("");
                self.handle_command(cmd, message, chatter).await;
            } else {
                let response = "Nothing".to_string();
                // let response = self.brain.respond(chatter, message).await;
                println!("Got the response sending it over chat: {}", response);
                // self.twitch_api_client.send_chat_message(response).await;
            }
        };
    }

    pub async fn handle_command(&self, cmd: &str, message: &str, chatter_user_name: &str) {
        match cmd {
            "!love" => {
                self.twitch_api_client
                    .send_chat_message(format!(
                        "Hi @{}, I love you and your beef cheeks",
                        chatter_user_name
                    ))
                    .await;
            }
            "!test" => {
                self.twitch_api_client
                    .send_chat_message(format!(
                        "{} typed the command: {} with rest of command: {}",
                        chatter_user_name, cmd, message
                    ))
                    .await;
            }
            "!analdischarge" => {
                self.twitch_api_client
                    .send_chat_message("You should probably put on a diaper".to_string())
                    .await;
            }
            _ => {
                let response = self.brain.respond(chatter_user_name, message).await;
                println!("Got the response sending it over chat: {}", response);

                self.twitch_api_client.send_chat_message(response).await;
            }
        }
    }

    pub async fn handle_ad_break_begin(&self, json: &Value) {
        let duration = &json["payload"]["event"]["duration_seconds"];
        self.twitch_api_client
            .send_chat_message(format!("{} second ad break starting!", duration))
            .await;
        println!("{}", json);
    }
}
