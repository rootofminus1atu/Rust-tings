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


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut conversation = CleverbotConversation::new(2);
    conversation.initialize_cookies().await?;

    let stim = "stimulus=asd&cb_settings_scripting=no&islearning=1&icognoid=wsf&icognocheck=5c98e0c77126e13c606adac5c0cc2124";
    //let res = conversation.sample_signal(stim).await?;

    //println!("{}", res);


    // Create a reqwest Client
    let client = Client::new();

    // Define the target URL
    let url = "https://www.cleverbot.com/webservicemin?uc=UseOfficialCleverbotAPI";

    // Create a HashMap for cookies (self.cookies in Python)
    let mut cookies = HashMap::new();
    cookies.insert("cookie_name", "cookie_value"); // Replace with your actual cookies

    // Define the payload as a string (payload in Python)
    let payload = "stimulus=asd&cb_settings_scripting=no&islearning=1&icognoid=wsf&icognocheck=5c98e0c77126e13c606adac5c0cc2124"; // Replace with your actual payload

    // Create a reqwest RequestBuilder for the POST request
    let request_builder = client
        .post(url)
        .header(COOKIE, "XVIS=TE1939AFFIAGAYQZN8T31") // Replace with your actual cookies
        .body(payload);

    // Send the POST request
    let response = request_builder.send().await?;

    // Check the response status code
    if response.status().is_success() {
        let response_text = response.text().await?;
        println!("Response: {}", response_text);
    } else {
        println!("Request failed with status code: {:?}", response.status());
    }

    Ok(())
}





