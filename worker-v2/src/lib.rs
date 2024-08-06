use worker::*;
use serde::{Serialize, Deserialize};
use reqwest::Client;

#[derive(Debug, Serialize, Deserialize)]
struct Greeting {
    name: String,
    message: String
}

// THIS is not possible, because both things live outside the crate
impl From<reqwest::Error> for worker::Error {
    fn from(error: reqwest::Error) -> Self {
        worker::Error::RustError(error.to_string())
    }
}

#[event(fetch)]
async fn fetch(
    req: Request,
    env: Env,
    ctx: Context,
) -> Result<Response> {
    let router = Router::new();

    router
        .get_async("/hello/:name", |req, ctx| async move {
            let name = ctx.param("name").unwrap().to_string();

            console_log!("we got name: `{}`", name);

            let message = format!("Hello, {}!", name);

            let g = Greeting { name, message };

            Response::from_json(&g)
        })
        .get_async("/external", |_, _| async move {
            let client = Client::new();
            let url = "https://api.example.com/data";

            let html = client.get(url)
                .send()
                .await?
                .error_for_status()?
                .text()
                .await?;

            Response::ok(html)
        })
        .run(req, env).await
}