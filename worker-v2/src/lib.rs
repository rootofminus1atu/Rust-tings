use worker::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Greeting {
    name: String,
    message: String
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
        .run(req, env).await
}