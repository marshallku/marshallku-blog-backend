use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscordAlert {
    pub embeds: Vec<DiscordEmbed>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscordEmbed {
    #[serde(rename = "type")]
    pub embed_type: String,
    pub title: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<i32>,
    pub fields: Vec<DiscordField>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<DiscordFooter>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscordField {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscordFooter {
    pub text: String,
}

pub async fn send_message(message: DiscordEmbed) -> Result<(), Box<dyn std::error::Error>> {
    let webhook_url = match std::env::var("DISCORD_WEBHOOK_URL") {
        Ok(url) => url,
        Err(e) => {
            log::error!("Failed to get DISCORD_WEBHOOK_URL: {}", e);
            return Err(e.into());
        }
    };

    let webhook_message = DiscordAlert {
        embeds: vec![message],
    };

    let client = reqwest::Client::new();
    let response = client
        .post(webhook_url)
        .json(&webhook_message)
        .send()
        .await?;

    if !response.status().is_success() {
        log::error!(
            "Failed to send webhook: {:?}, {}",
            webhook_message,
            response.status()
        );
        return Err(format!("Failed to send webhook: {}", response.status()).into());
    }

    Ok(())
}
