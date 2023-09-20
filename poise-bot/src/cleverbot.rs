use std::collections::VecDeque;
type Error = Box<dyn std::error::Error + Send + Sync>;

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



pub struct Cleverbot {
    pub api_link: String,
    pub api_key: String,
    pub cookie: String,
    pub history: MaxQueue<String>,
}

impl Cleverbot {
    pub fn new(api_key: String, api_link: String, max: usize) -> Cleverbot {
        Cleverbot {
            api_link: api_link,
            api_key: api_key,
            cookie: Cleverbot::generate_cookie(),
            context: MaxQueue::new(max),
        }
    }

    fn generate_cookie() -> String {
        "hi".to_string()
    }

    fn get_response(&self, stimulus: &str) -> Result<String, Error> {
        println!("{:#?}", self);

        let response = Client::new()
            .post(&self.api_link)
            .header("cookie", &self.cookie)
            .header("clevreq-api-key", &self.api_key)
            .body(serde_json::to_string(&json!({
                "stimulus": stimulus,
                "context": self.context.get_all(),
            }))?)
            .send()
            .await?
            .text()
            .await?;

    Ok(response)
    }
}