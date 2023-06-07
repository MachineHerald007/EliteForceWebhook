use std::env;
use dotenv::dotenv;
use reqwest::Error;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: String,
    username: String,
    avatar: String,
    discriminator: String,
    #[serde(rename = "publicFlags")]
    public_flags: usize,
    flags: usize,
    banner: Option<String>,
    #[serde(rename = "accentColor")]
    accent_color: Option<usize>,
    #[serde(rename = "globalName")]
    global_name: Option<String>,
    #[serde(rename = "avatarDecoration")]
    avatar_decoration: Option<String>,
    #[serde(rename = "")]
    display_name: Option<String>,
    #[serde(rename = "bannerColor")]
    banner_color: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
struct PartialGuildObject {}

#[derive(Debug, Serialize, Deserialize)]
struct PartialChannelObject {}

#[derive(Debug, Serialize, Deserialize)]
struct WebhookObject {
    id: String,
    r#type: u32,
    #[serde(rename = "guildId")]
    guild_id: String,
    #[serde(rename = "channelId")]
    channel_id: String,
    user: User,
    name: String,
    avatar: String,
    token: String,
    #[serde(rename = "applicationId")]
    application_id: String,
    #[serde(rename = "sourceGuild")]
    source_guild: PartialGuildObject,
    #[serde(rename = "sourceChannel")]
    source_channel: PartialChannelObject,
    url: String
}

#[derive(Debug, Deserialize, Serialize)]
struct ExecuteWebhook {
    content: String
}

async fn get_webhook_obj() -> Result<(), Error> {
    let api_endpoint = std::env::var("API_ENDPOINT").expect("API_ENDPOINT must be set.");
    let webhook_id = std::env::var("BANNER_UPDATES_WEBHOOK_ID").expect("WEBHOOK_ID must be set.");
    let url = api_endpoint.to_owned() + &webhook_id;

    let response: WebhookObject =
        reqwest::get(url)
        .await?
        .json()
        .await?
    ;

    println!("{:?}", response);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    let args: Vec<String> = env::args().collect();
    let api_endpoint = std::env::var("API_ENDPOINT").expect("API_ENDPOINT must be set.");
    let webhook_id = std::env::var("BANNER_UPDATES_WEBHOOK_ID").expect("WEBHOOK_ID must be set.");
    let webhook_token = std::env::var("BANNER_UPDATES_WEBHOOK_TOKEN").expect("WEBHOOK_TOKEN must be set.");
    let url = api_endpoint.to_owned() + &webhook_id + "/" + &webhook_token;
    
    //This is the JSON payload we're sending to Discord Webhook
    let banner_obj = ExecuteWebhook {
        content: String::from(&args[1])
    };

    let client = reqwest::Client::new();
    let response =
        client.post(url)
        .header("Content-Type", "application/json")
        .json(&banner_obj)
        .send()
        .await?
    ;

    Ok(())
}