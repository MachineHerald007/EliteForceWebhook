#![windows_subsystem = "windows"]

use std::env;
use dotenv::dotenv;
use reqwest::Error;
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
struct DiscordPayload {
    content: String
}

async fn send_banners(webhook_urls: Vec<String>, banner_obj: DiscordPayload) -> Result<(), Error> {
    for url in webhook_urls {
        let client = reqwest::Client::new();
        let response =
            client.post(url)
            .header("Content-Type", "application/json")
            .json(&banner_obj)
            .send()
            .await?
        ;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    let api_endpoint = std::env::var("API_ENDPOINT").expect("API_ENDPOINT must be set.");
    
    let black_order_id = std::env::var("BLACK_ORDER_WEBHOOK_ID").expect("WEBHOOK_ID must be set.");
    let black_order_token = std::env::var("BLACK_ORDER_WEBHOOK_TOKEN").expect("WEBHOOK_TOKEN must be set.");

    let kitsune_id = std::env::var("KITSUNE_WEBHOOK_ID").expect("WEBHOOK_ID must be set.");
    let kitsune_token = std::env::var("KITSUNE_WEBHOOK_TOKEN").expect("WEBHOOK_TOKEN must be set.");

    let mut webhook_urls = Vec::new();
    webhook_urls.push(api_endpoint.to_owned() + &black_order_id + "/" + &black_order_token);
    webhook_urls.push(api_endpoint.to_owned() + &kitsune_id + "/" + &kitsune_token);

    let args: Vec<String> = env::args().collect();
    let banner_obj = DiscordPayload {
        content: String::from(&args[1])
    };

    send_banners(webhook_urls, banner_obj).await?;

    Ok(())
}