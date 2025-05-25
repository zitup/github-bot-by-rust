use std::env;

use dotenv::dotenv;
use reqwest::Error;
use serde_json::json;

pub struct TelegramClient {
    client: reqwest::Client,
    token: String,
}

impl TelegramClient {
    pub async fn new() -> Self {
        dotenv().ok();
        let token = env::var("TELEGRAM_TOKEN").unwrap();
        let client = reqwest::Client::new();
        Self { client, token }
    }

    pub async fn send_message(&self, message: String, chat_id: &str) -> Result<(), Error> {
        let url = format!("https://api.telegram.org/bot{}/sendMessage", self.token);
        self.client
            .post(url)
            .header("Content-Type", "application/json")
            .json(&json!({
                "chat_id": chat_id,
                "text": message,
                "link_preview_options": json!({
                  "is_disabled": true
                })
            }))
            .send()
            .await?;
        Ok(())
    }
}
