use serde_json::Value;

pub async fn handle(json: &Value) {
    let message_type = &json["metadata"]["message_type"];
    let event = &json["payload"]["event"];
    println!("{}: {}", message_type, event);
}
