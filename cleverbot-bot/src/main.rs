#![allow(dead_code)]
#![allow(unused_imports)]

mod utils {
    pub mod queue;
}

mod cleverbot;
use cleverbot::CleverbotConversation;
use regex::Error;
use serde_json::json;

use std::collections::HashMap;
use std::process::{Command, Stdio};
use std::vec;
use reqwest::header::COOKIE;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use reqwest::blocking;

use chrono::prelude::*;
use regex::Regex;


#[derive(Debug)]
#[derive(Deserialize, Serialize)]
struct ChuckNorrisJoke {
    value: String,
}




use std::collections::VecDeque;

#[derive(Debug)]
pub struct MaxQueue<T> {
    max: usize,
    list: VecDeque<T>,
}

impl<T> MaxQueue<T> {
    pub fn new(max: usize) -> MaxQueue<T> {
        MaxQueue {
            max: max,
            list: VecDeque::with_capacity(max),
        }
    }

    pub fn push_front(&mut self, item: T) {
        if self.list.len() == self.max {
            self.list.pop_back();
        }
        self.list.push_front(item);
    }

    pub fn push_back(&mut self, item: T) {
        if self.list.len() == self.max {
            self.list.pop_front();
        }
        self.list.push_back(item);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.list.pop_front()
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.list.pop_back()
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub fn clear(&mut self) {
        self.list.clear();
    }

    pub fn get_all(&self) -> &VecDeque<T> {
        &self.list
    }
}


#[derive(Debug)]
pub struct Cleverbot {
    pub api_link: String,
    pub api_key: String,
    pub cookie: String,
    pub context: MaxQueue<String>,
}

impl Cleverbot {
    pub fn new(api_key: String, api_link: String, cookie: String, max: usize) -> Self {
        Cleverbot {
            api_link: api_link,
            api_key: api_key,
            cookie: cookie,
            context: MaxQueue::new(max),
        }
    }

    fn update_context(&mut self, stimulus: &str, response: &str) {
        self.context.push_front(stimulus.to_string());
        self.context.push_front(response.to_string());
    }

    async fn get_response(&mut self, stimulus: &str) -> Result<String, Box<dyn std::error::Error>> {
        let req_params = serde_json::to_string(&json!({
            "stimulus": stimulus,
            "context": self.context.get_all(),
        }))?;

        let response = Client::new()
            .post(&self.api_link)
            .header("cookie", &self.cookie)
            .header("clevreq-api-key", &self.api_key)
            .body(req_params)
            .send()
            .await?
            .text()
            .await?;

        let cleared_response = response.trim_matches('"');

        self.update_context(stimulus, cleared_response);
    Ok(response)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let mut cleverbot = Cleverbot::new(
        "key".to_string(),
        "no".to_string(),
        "nuh".to_string(),
        10,
    );

    let res = cleverbot.get_response("Are you a bot?").await?;

    println!("Got response: {}", res);

    
    let res = cleverbot.get_response("you are wrong...").await?;

    println!("Got response: {}", res);

    println!("{:#?}", cleverbot);


    Ok(())
}




