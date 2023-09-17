use std::collections::HashMap;
use chrono::prelude::*;


use regex::Regex;

use crate::utils::queue::Queue;

pub struct CleverbotConversation {
    cookies: HashMap<String, String>,
    context_queue: Queue<String>,
}

impl CleverbotConversation {
    pub fn new(max_context: usize) -> CleverbotConversation {
        CleverbotConversation {
            cookies: HashMap::new(),
            context_queue: Queue::new(max_context),
        }
    }

    fn get_date(&self) -> String {
        let now: DateTime<Utc> = chrono::Utc::now();
        format!("{}{}{}", now.year(), now.month(), now.day())
    }

    pub async fn initialize_cookies(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("https://www.cleverbot.com/extras/conversation-social-min.js?{}", self.get_date());

        let body = reqwest::get(url).await?;
        let headers = body.headers();

        let the_cookie = headers
            .get("set-cookie")
            .ok_or("Couldn't find field named \"set-cookie\" :(")?
            .to_str()?;


        let re = Regex::new(r"XVIS=([^;%]+)").unwrap();

        let extracted_value = re.captures(the_cookie)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str())
            .ok_or("Couldn't find XVIS cookie :(")?;

        self.cookies.insert("XVIS".to_string(), extracted_value.to_string());

        Ok(())
    }

    fn build_payload(&self, stimulus: &str) -> String {
        let mut payload = String::new();
        payload.push_str(&format!("stimulus={}", stimulus));


        for (i, ctx) in self.context_queue.get_all().iter().rev().enumerate() {
            payload.push_str(format!("stimulus{}", i).as_str());
        }

        payload
    }


    pub async fn sample_signal(&self, stimulus: &str) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("https://www.cleverbot.com/webservicemin?uc=UseOfficialCleverbotAPI&",);
        let cookie = "hi";

        let body = reqwest::Client::new()
            .post(url)
            .header("cookie", cookie)
            .body(stimulus.to_string())
            .send()
            .await?
            .text()
            .await?;

        Ok(body)
    }
}