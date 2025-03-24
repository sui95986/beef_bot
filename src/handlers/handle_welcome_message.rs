use crate::clients::TwitchApiClient;
use serde_json::Value;

pub async fn handle(json: &Value) {
    let session_id = &json["payload"]["session"]["id"];

    println!("The session Id is: {}", session_id);

    let client = TwitchApiClient::new();

    let result = client.subscribe_to_chat_events(&session_id);

    let response = result
        .await
        .expect("Call to subscribe to chat messages errored");

    println!(
        "Response when register to listen to chat messages: {}",
        response.status()
    );
}
