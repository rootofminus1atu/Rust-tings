#![allow(dead_code)]
#![allow(unused_imports)]

mod utils {
    pub mod queue;
}

mod cleverbot;
use cleverbot::CleverbotConversation;
// imports work here

use std::collections::HashMap;

use reqwest::header::COOKIE;
use reqwest::Client;

use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
struct ChuckNorrisJoke {
    value: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let chuck_api_url = "https://api.chucknorris.io/jokes/random";

    let joke: ChuckNorrisJoke = reqwest::get(chuck_api_url)
        .await?
        .json()
        .await?;

    println!("body = {:?}", joke);



    let gen_joke: serde_json::Value = reqwest::get(chuck_api_url)
        .await?
        .json()
        .await?;

    println!("body = {:?}", gen_joke);

    let gen_joke_val = gen_joke.get("value").unwrap().as_str().unwrap();
    println!("body = {:?}", gen_joke_val);



    let url = "https://www.cleverbot.com/webservicemin?uc=UseOfficialCleverbotAPI";
    let payload = "stimulus=hello&cb_settings_scripting=no&islearning=1&icognoid=wsf&icognocheck=421e34f1f2a875aae4ed736e04cb1265";
    let cookie = "XVIS=TE1939AFFIAGAYQZN8T31";

    let client = Client::new();
    let request_builder = client
        .post(url)
        .header(COOKIE, cookie) 
        .body(payload);

    let response = request_builder.send().await?;

    if response.status().is_success() {
        let response_text = response.text().await?;
        println!("Response: {}", response_text);
    } else {
        println!("Request failed with status code: {:?}", response.status());
    }

    Ok(())
}




