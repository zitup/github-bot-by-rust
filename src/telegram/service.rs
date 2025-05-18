use std::env;

use super::api::TelegramClient;
use dotenv::dotenv;
use reqwest::Error;

pub struct TelegramService {
    client: TelegramClient,
    chat_id: String,
}

impl TelegramService {
    pub async fn new() -> Self {
        dotenv().ok();
        let chat_id = env::var("TELEGRAM_CHAT_ID").unwrap();
        let client = TelegramClient::new().await;
        Self { client, chat_id }
    }

    pub async fn send_message(&self, message: &str) -> Result<(), Error> {
        self.client.send_message(message, &self.chat_id).await
    }
}
