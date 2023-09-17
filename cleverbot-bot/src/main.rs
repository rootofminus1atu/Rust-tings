#![allow(dead_code)]
#![allow(unused_imports)]

mod utils {
    pub mod queue;
}

mod cleverbot;
use cleverbot::CleverbotConversation;
use serde_json::json;

use std::collections::HashMap;
use std::process::{Command, Stdio};
use std::vec;
use reqwest::header::COOKIE;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
struct ChuckNorrisJoke {
    value: String,
}

fn get_clevereq_path() -> PathBuf {
    let mut adaptable_path = PathBuf::from(".");
    adaptable_path.push("src");

    if cfg!(windows) {
        adaptable_path.push("clevreq.exe");
    } else {
        adaptable_path.push("clevreq");
    };

    adaptable_path
}

fn get_cleverbot_response() -> Result<String, String> {
    // Define the command to run the Python script
    let python_exe = get_clevereq_path(); // Replace with the actual path
    let cookie = "";
    let payload = "";

    // Execute the Python script as a subprocess
    let output = Command::new(python_exe)
        .arg("--cookie")
        .arg(cookie)
        .arg("--payload")
        .arg(payload)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to execute command");

    // Check if the Python script executed successfully
    if output.status.success() {
        // Capture and print the output of the Python script
        let stdout = String::from_utf8_lossy(&output.stdout);
        return Ok(stdout.to_string());
    } else {
        // Capture and print any error messages
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(stderr.to_string());
    }


}

use std::path::{Path, PathBuf};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // time to use paths and pthbufs to get to the python executable
    // remember that linux and windows are different

    // let response = get_cleverbot_response()?;
    // println!("response is: {}", response);
    let api_key = "lol";
    let cookie = "hi";
    let api_str = "";
        
    let res = Client::new()
        .post(api_str)
        .header("cookie", cookie)
        .header("clevreq-api-key", api_key)
        .body(serde_json::to_string(&json!({
            "stimulus": "nah",
            "context": vec![ "do you wanna build a snowman?", "hi" ],
        }))?)
        .send()
        .await?;

    println!("{}", res.text().await?);


    Ok(())
}




