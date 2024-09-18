use max_queue::MaxQueue;
use tokio::sync::RwLock as AsyncRwLock;
use std::sync::Arc;
use chrono::{DateTime, Utc};
use md5::{Md5, Digest};
mod max_queue;
use percent_encoding::{utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};

const PYTHONIC_NON_ALPHANUMERIC: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'!')
    .remove(b'#')
    .remove(b'$')
    .remove(b'%')
    .remove(b'&')
    .remove(b'\'')
    .remove(b'(')
    .remove(b')')
    .remove(b'*')
    .remove(b'+')
    .remove(b',')
    .remove(b'/')
    .remove(b':')
    .remove(b';')
    .remove(b'=')
    .remove(b'?')
    .remove(b'@')
    .remove(b'[')
    .remove(b']')
    .remove(b'~');


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    
    let c = CleverbotBuilder::new()
        .with_client(reqwest::Client::new())
        .build()
        .await?;

    c.context_queue.write().await.push_back("are you a bot !#$%&'()*+,/:;=?@[]~ \\\n\tetcEND".into());
    c.context_queue.write().await.push_back("no !#$&'()*+,/:;=?@[]~ END".into());
    c.context_queue.write().await.push_back("how are you".into());
    c.context_queue.write().await.push_back("!#$%&'()*+,/:;=?@[]~\\\n\t\"<>^`{|}\x00\x01\x02\x03\x04\x05\x06\x07\x08\x0B\x0C\x0E\x0F\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1A\x1B\x1C\x1D\x1E\x1F\x7F".into());

    let p = c.build_payload("some stim :) !#$%&'()*+,/:;=?@[]~ END").await;
    dbg!(&p);


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

/// TIL that python's requests' `requests.utils.requote_uri(s)` uses "!#$%&'()*+,/:;=?@[]~" as safe chars, ok i guess
fn pythonic_encode(input: &str) -> String {
    utf8_percent_encode(input, PYTHONIC_NON_ALPHANUMERIC).to_string()
}

impl Cleverbot {
    // pub fn new() -> CleverbotBuilder {
    //     println!("use the builder next time");
    //     CleverbotBuilder::new()
    // }
    async fn build_payload(&self, stimulus: &str) -> String {
        let stimulus_str = format!("stimulus={}", pythonic_encode(stimulus));

        let context_str = self.context_queue.read().await
            .get_all()
            .iter()
            .rev()
            .enumerate()
            .map(|(i, text)| format!("&vText{}={}", i + 2, pythonic_encode(text)))
            .collect::<String>();

        let partial_payload = format!("{}{}", stimulus_str, context_str);

        let hash_input = &partial_payload[7..33];
        let mut hasher = Md5::new();
        hasher.update(hash_input.as_bytes());
        let finalized = hasher.finalize();
        let magic_ingredient = format!("{:x}", finalized);

        let junk_str = format!("&cb_settings_scripting=no&islearning=1&icognoid=wsf&icognocheck={}", magic_ingredient);

        let payload = format!("{}{}", partial_payload, junk_str);

        payload
    }

    pub async fn get_response(&self) {
        todo!()
    }
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