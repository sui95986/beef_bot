use serde_json::Value;

pub async fn handle(json: &Value) {
    let message_type = &json["metadata"]["message_type"];
    let event = &json["payload"]["event"];
    let message_type_as_string = message_type.as_str().unwrap_or("");
    if !message_type_as_string.contains("session_keepalive") {
        println!("{}: {}", message_type, event);
    }
}
