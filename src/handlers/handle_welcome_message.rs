use dotenv_codegen::dotenv;
use serde_json::{Value, json};

pub async fn handle(json: &Value) {
    let session_id = &json["payload"]["session"]["id"];
    let api_token = dotenv!("OAUTH_TOKEN");
    let client_id = dotenv!("CLIENT_ID");
    let bot_user_id = dotenv!("BOT_USER_ID");
    let broadcaster_id = dotenv!("BROADCASTER_ID");

    println!("The session Id is: {}", session_id);

    let client = reqwest::Client::new();

    let request_body = get_request_body(broadcaster_id, bot_user_id, session_id);
    let result = client
        .post("https://api.twitch.tv/helix/eventsub/subscriptions")
        .bearer_auth(api_token)
        .header("Client-Id", client_id)
        .json(&request_body)
        .send()
        .await;

    let response = result.expect("Call to subscribe to chat messages errored");

    println!(
        "Response when register to listen to chat messages: {}",
        response.status()
    );
}

pub fn get_request_body(broadcaster_id: &str, bot_user_id: &str, session_id: &Value) -> Value {
    let json = json!({
        "type": "channel.chat.message",
        "version": "1",
        "condition": {
            "broadcaster_user_id": broadcaster_id,
            "user_id": bot_user_id,
        },
        "transport": {
            "method": "websocket",
            "session_id": session_id
        }
    });
    json
}
