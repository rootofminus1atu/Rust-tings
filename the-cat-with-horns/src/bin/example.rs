#![allow(dead_code)]

use poise::serenity_prelude::Timestamp;
use reqwest;
use tokio;
use ordinal::Ordinal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    Ok(())
}

async fn date_test() -> Result<(), Box<dyn std::error::Error>> {
    let ts = Timestamp::now();
    
    let day = ts.format("%d").to_string().parse::<i32>();
    let day_ordinal = Ordinal(day.unwrap()).to_string();
    let month = ts.format("%B");
    let year = ts.format("%Y");

    println!("{} {} {}", month, day_ordinal, year);

    Ok(())
}

async fn animal_test() -> Result<(), Box<dyn std::error::Error>> {
    let img = fetch_animal_img("https://randomfox.ca/floof", "image").await?;

    println!("{:?}", img);

    Ok(())
}

async fn fetch_animal_img(url: &str, field_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let img: String = reqwest::get(url)
        .await?
        .json::<serde_json::Value>()
        .await?
        .get(field_name)
        .and_then(|value| value.as_str())
        .ok_or("Field not found or not a string")?
        .to_string();

    Ok(img)
}