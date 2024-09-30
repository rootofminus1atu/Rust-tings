use max_queue::MaxQueue;
use tokio::sync::RwLock as AsyncRwLock;
use std::sync::Arc;
use chrono::{DateTime, Utc};
mod max_queue;
use percent_encoding::{utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};
use std::str::{self, Utf8Error};

// TODO:
// - add params to the errors
// - docs
// - examples
// - turn prints into tracing::debug!()
// - tests? but how? like the responses will differ since it's an api request to an ai that is statistical not deterministic
// - rewrite the logic of the respond() fn for better code flow and less code duplications

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    

    let c = CleverbotBuilder::default()
        .with_retries(true)
        .build()
        .await?;

    let r = c.get_response("are you a bot").await?;
    println!("h");
    println!("response: {r}");
    println!("h");

    return Ok(());


    let c = CleverbotBuilder::default()
        .with_client(reqwest::Client::new())
        .build()
        .await?;

    c.history.write().await.push_back("are you a bot !#$%&'()*+,/:;=?@[]~ \\\n\tetcEND".into());
    c.history.write().await.push_back("no !#$&'()*+,/:;=?@[]~ END".into());
    c.history.write().await.push_back("how are you".into());
    c.history.write().await.push_back("!#$%&'()*+,/:;=?@[]~\\\n\t\"<>^`{|}\x00\x01\x02\x03\x04\x05\x06\x07\x08\x0B\x0C\x0E\x0F\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1A\x1B\x1C\x1D\x1E\x1F\x7F".into());

    let p = c.build_payload("some stim :) !#$%&'()*+,/:;=?@[]~ END").await;
    dbg!(&p);


    Ok(())
}


macro_rules! remove_many {
    ($set:expr, [$($char:literal),*]) => {
      ($set$(.remove($char))*)  
    };
}
/// pythonic safe chars: `!#$%&'()*+,/:;=?@[]~`
const PYTHONIC_NON_ALPHANUMERIC: &AsciiSet = &remove_many!(
    NON_ALPHANUMERIC, 
    [b'!', b'#', b'$', b'%', b'&', b'\'', b'(', b')', b'*', b'+', b',', b'/', b':', b';', b'=', b'?', b'@', b'[', b']', b'~']
);


#[derive(Debug, thiserror::Error)]
enum CleverbotError {
    #[error("no cookie found")]
    NoCookieFound,
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error("Could not decode the response bytes: {0}")]
    Utf8Error(#[from] Utf8Error),
    #[error("Invalid response from Cleverbot API")]
    InvalidResponseFromCleverbotApi,
    #[error("Bad response from Cleverbot API: {0}")]
    BadResponse(String),
    #[error("Bad response from Cleverbot API after retrying: {0}")]
    BadResponseAfterRetrying(String),
}

/// Cleverbot client for interacting with the Cleverbot API.
#[derive(Debug, Clone)]
struct Cleverbot {
    cookie: Arc<AsyncRwLock<String>>,
    history: Arc<AsyncRwLock<MaxQueue<String>>>,
    client: reqwest::Client,
    with_retries: bool,
}

impl Cleverbot {
    pub async fn get_response(&self, stimulus: &str) -> Result<String, CleverbotError> {
        let payload = self.build_payload(stimulus).await;
        let answer = self.send_cleverbot_request(&payload).await?;

        if !Self::is_bad_cleverbot_response(&answer) {
            (*self.history.write().await).push_front(answer.clone());
            return Ok(answer)
        }

        if !self.with_retries {
            return Err(CleverbotError::BadResponse(answer));
        }

        // now we retrying
        let new_cookie = get_cookie(&self.client).await?;
        *self.cookie.write().await = new_cookie;
        let new_answer = self.send_cleverbot_request(&payload).await?;

        if !Self::is_bad_cleverbot_response(&new_answer) {
            (*self.history.write().await).push_front(new_answer.clone());
            return Ok(new_answer)
        }

        Err(CleverbotError::BadResponseAfterRetrying(new_answer))
    }

    async fn build_payload(&self, stimulus: &str) -> String {
        let stimulus_str = format!("stimulus={}", pythonic_encode(stimulus));

        let context_str = self.history.read().await
            .get_all()
            .iter()
            .rev()
            .enumerate()
            .map(|(i, text)| format!("&vText{}={}", i + 2, pythonic_encode(text)))
            .collect::<String>();

        let cb_settings_str = "&cb_settings_scripting=no&islearning=1&icognoid=wsf&icognocheck=";

        let partial_payload = format!("{}{}{}", stimulus_str, context_str, cb_settings_str);

        let magic_ingredient = format!("{:x}", md5::compute(&partial_payload[7..33]));

        let payload = format!("{}{}", partial_payload, magic_ingredient);

        tracing::debug!("clevreq payload: {payload}");

        payload
    }

    async fn send_cleverbot_request(&self, payload: &str) -> Result<String, CleverbotError> {
        let bytes_res = self.client.post("https://www.cleverbot.com/webservicemin?uc=UseOfficialCleverbotAPI")
            .body(payload.to_string())
            .header("cookie", &self.cookie.read().await.clone())
            // .header("accept-encoding", "gzip, deflate")
            .header("user-agent", "python-requests/2.32.3")
            .send()
            .await?
            .bytes()
            .await?;

        tracing::debug!("clevreq bytes_res: {:?}", bytes_res);
        
        let text = str::from_utf8(&bytes_res)?;
        let response = text.split('\r').next().ok_or(CleverbotError::InvalidResponseFromCleverbotApi)?;

        Ok(response.into())
    }

    fn is_bad_cleverbot_response(response: &str) -> bool {
        match response {
            "Hello from Cleverbot\n" | "<html" => true,
            _ => false
        }
    }
}

/// Builder for constructing a `Cleverbot` instance.
/// 
/// 
struct CleverbotBuilder {
    client: reqwest::Client,
    with_retries: bool,
    history_size: usize,
}

impl Default for CleverbotBuilder {
    fn default() -> Self {
        Self { 
            client: reqwest::Client::new(), 
            with_retries: true, 
            history_size: Self::DEFAULT_QUEUE_SIZE 
        }
    }
}

impl CleverbotBuilder {
    const DEFAULT_QUEUE_SIZE: usize = 50;

    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = client;
        self
    }

    /// Enable or disable retries. By default, retries are enabled.
    pub fn with_retries(mut self, with_retries: bool) -> Self {
        self.with_retries = with_retries;
        self
    }

    /// Use a custom history size for storing the chat history.
    pub fn with_custom_history_size(mut self, history_size: usize) -> Self {
        self.history_size = history_size;
        self
    }

    pub async fn build(self) -> Result<Cleverbot, CleverbotError> {
        let cookie = get_cookie(&self.client).await?;
        tracing::debug!("cookie: {cookie}");

        Ok(Cleverbot {
            cookie: Arc::new(AsyncRwLock::new(cookie)),
            history: Arc::new(AsyncRwLock::new(MaxQueue::<String>::new(self.history_size))),
            client: self.client,
            with_retries: self.with_retries
        })
    }
}


async fn get_cookie(client: &reqwest::Client) -> Result<String, CleverbotError> {
    let url = format!("https://www.cleverbot.com/extras/conversation-social-min.js?{}", get_date());
    let resp = client.get(&url).send().await?;

    let cookie_before = resp.headers()
        .get("set-cookie")
        .and_then(|s| s.to_str().ok())
        .and_then(|s| s.split(';').next());

    let cookie_str = cookie_before
        .map(|s| s.replace("B%", "32"));  // i have no idea why 31 ore 32 work, but other ones don't

    cookie_str.ok_or(CleverbotError::NoCookieFound)
}

fn get_date() -> String {
    let now: DateTime<Utc> = Utc::now();
    now.format("%Y%m%d").to_string()
}

/// TIL that python's requests' `requests.utils.requote_uri(s)` uses "!#$%&'()*+,/:;=?@[]~" as safe chars, ok i guess
fn pythonic_encode(input: &str) -> String {
    utf8_percent_encode(input, PYTHONIC_NON_ALPHANUMERIC).to_string()
}
