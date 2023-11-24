use std::env;
use serde_json::{json, Value};

pub async fn send_one_message(message_text: String, chat_id: Option<String>, bot_token: Option<String>) {
    let formatted_message = message_text
        .replace(".", "\\.")
        .replace("(", "\\(")
        .replace(")", "\\)");

    let chat_id_env_var = env::var(chat_id.unwrap_or("TELEGRAM_CHAT_ID".parse().unwrap())).unwrap();
    let token = env::var(bot_token.unwrap_or("TELEGRAM_TOKEN".parse().unwrap())).unwrap();

    let body: Value;

    if chat_id_env_var.contains("/") {
        let chat_info = chat_id_env_var.split("/").collect::<Vec<&str>>();
        let chat_id = chat_info.get(0).unwrap();
        let reply_to_message_id = chat_info.get(1).unwrap();
        body = json!({
            "chat_id": chat_id,
            "text": formatted_message,
            "parse_mode": "MarkdownV2",
            "reply_to_message_id": reply_to_message_id
        });
    } else {
        body = json!({
            "chat_id": chat_id_env_var,
            "text": formatted_message,
            "parse_mode": "MarkdownV2",
        });
    };

    println!("[Messenger] Sending one message (formatted) >> {body}\n");

    let client = reqwest::Client::new();
    // https://core.telegram.org/bots/api#available-methods
    let response = client.post(&format!("https://api.telegram.org/bot{token}/sendMessage"))
        .json(&body)
        .send()
        .await;

    if response.is_ok() {
        println!("[Messenger] Message sent successfully!")
    } else {
        eprintln!("[Messenger] Failed to send message. Status: {:?}", response);
    }
}
