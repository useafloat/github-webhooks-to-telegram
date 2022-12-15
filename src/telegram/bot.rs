use reqwest;
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;

use super::messages::Message;

#[derive(Serialize)]
struct PhotoMedia {
    #[serde(rename = "type")]
    kind: String,
    media: String,
}

fn get_text_body_map<'a>(message: &'a str, chat_id: &'a str) -> HashMap<&'a str, &'a str> {
    HashMap::from([
        ("chat_id", chat_id),
        ("parse_mode", "MarkdownV2"),
        ("disable_web_page_preview", "true"),
        ("text", message),
    ])
}

fn get_photo_body_map<'a>(photo: &'a str, chat_id: &'a str) -> HashMap<&'a str, &'a str> {
    HashMap::from([("chat_id", chat_id), ("photo", photo)])
}

fn get_media_group_body_json(message: Message, chat_id: &str) -> serde_json::Value {
    let media: Vec<PhotoMedia> = message
        .images
        .into_iter()
        .map(|image| PhotoMedia {
            kind: "photo".to_string(),
            media: image,
        })
        .collect();

    json!({
        "chat_id": chat_id,
        "media": media
    })
}

pub async fn post_chat_message(message: Message) -> Result<String, Box<dyn std::error::Error>> {
    let telegram_bot = std::env::var("TELEGRAM_BOT").expect("Missing env variable TELEGRAM_BOT");
    let telegram_chat_id =
        std::env::var("TELEGRAM_CHAT_ID").expect("Missing env variable TELEGRAM_CHAT_ID");

    let client = reqwest::Client::new();

    let response = client
        .post(format!(
            "https://api.telegram.org/bot{}/sendMessage",
            telegram_bot
        ))
        .json(&get_text_body_map(&message.text, &telegram_chat_id))
        .send()
        .await?;

    if message.images.len().eq(&1) {
        for image in message.images {
            println!("{:?}", image);
            let response = client
                .post(format!(
                    "https://api.telegram.org/bot{}/sendPhoto",
                    telegram_bot
                ))
                .json(&get_photo_body_map(&image, &telegram_chat_id))
                .send()
                .await?;

            println!("{:#?}", response);
        }
    } else if message.images.len().gt(&1) {
        let response = client
            .post(format!(
                "https://api.telegram.org/bot{}/sendMediaGroup",
                telegram_bot
            ))
            .json(&get_media_group_body_json(message, &telegram_chat_id))
            .send()
            .await?;

        println!("{:#?}", response);
    }
    Ok(response.text().await?)
}
