use max_queue::MaxQueue;
use tokio::sync::RwLock as AsyncRwLock;
use std::{error, sync::Arc};
use chrono::{DateTime, Utc};

mod max_queue;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let cookie = "some str";
    
    let c = CleverbotBuilder::new()
        .with_client(reqwest::Client::new())
        .build()
        .await?;


    println!("Hello, world!");

    Ok(())
}

#[derive(Debug, thiserror::Error)]
enum CleverbotError {
    #[error("no cookie found")]
    NoCookieFound,
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
}

#[derive(Debug, Clone)]
struct Cleverbot {
    cookie: Arc<AsyncRwLock<String>>,
    context_queue: Arc<AsyncRwLock<MaxQueue<String>>>,
    client: reqwest::Client
}

impl Cleverbot {
    // pub fn new() -> CleverbotBuilder {
    //     println!("use the builder next time please");
    //     CleverbotBuilder::new()
    // }
}

struct CleverbotBuilder {
    client: Option<reqwest::Client>,
}

impl CleverbotBuilder {
    pub fn new() -> Self {
        Self { client: None }
    }

    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub async fn build(self) -> Result<Cleverbot, CleverbotError> {
        let client = self.client.unwrap_or_else(reqwest::Client::new);
        let cookie = get_cookie(client.clone()).await?;

        Ok(Cleverbot {
            cookie: Arc::new(AsyncRwLock::new(cookie)),
            context_queue: Arc::new(AsyncRwLock::new(MaxQueue::<String>::new(50))),
            client
        })
    }
}



async fn get_cookie(client: reqwest::Client) -> Result<String, CleverbotError> {
    let url = format!("https://www.cleverbot.com/extras/conversation-social-min.js?{}", get_date());
    let resp = client.get(&url).send().await?;

    let cookie_before = resp.headers()
        .get("set-cookie")
        .and_then(|s| s.to_str().ok())
        .and_then(|s| s.split(';').next());

    let cookie_str = cookie_before
        .map(|s| s.replace("B%", "31"));  // i have no idea why 31 works, but it's the only one that does

    // info!("new cookie before: {:?}", cookie_before);
    // info!("new cookie after:  {:?}", cookie_str);

    cookie_str.ok_or(CleverbotError::NoCookieFound)
}

fn get_date() -> String {
    let now: DateTime<Utc> = Utc::now();
    println!("now: {}", now.format("%Y%m%d").to_string());
    now.format("%Y%m%d").to_string()
}