use anyhow::Context as _;
use poise::serenity_prelude as serenity;
use shuttle_secrets::SecretStore;
use shuttle_poise::ShuttlePoise;
use serde::{Deserialize, Serialize};
use poise::Event;

use std::path::PathBuf;

use tracing::info;
use reqwest::Client;
use serde_json::json;

struct Data {
    static_folder: PathBuf,
    api_link: String,
    api_key: String,
} 
// User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;



/// Responds with "world!"
#[poise::command(slash_command)]
async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("world!").await?;
    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
struct ChuckJoke {
    value: String,
}

/// Responds with a random Chuck Norris joke
#[poise::command(slash_command)]
async fn chuck(ctx: Context<'_>) -> Result<(), Error> {
    let joke: ChuckJoke = reqwest::get("https://api.chucknorris.io/jokes/random")
        .await?
        .json()
        .await?;

    ctx.say(joke.value).await?;

    Ok(())
}






async fn get_clevreq(msg: &str) -> Result<String, Error> {
    let api_key = "lol";

    // add error handling
    // TODO:
    // clean up the code, divide it into files
    // get rid of the " in the response, possibly in the fastapi
    // struct-ify, generate the cookie
    // struct-ify, it's important
    // some error handling in case cleverbot dies
    // add context
    // add sessions for each server/dm
    // something something db
    // add authentication key generation and better security (fastapi)
    // improve fastapi 
    
        
    let response = Client::new()
        .post()
        .header("cookie", )
        .header("clevreq-api-key", api_key)
        .body(serde_json::to_string(&json!({
            "stimulus": msg,
            "context": Vec::<String>::new(),
        }))?)
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}



#[shuttle_runtime::main]
async fn poise(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
    #[shuttle_static_folder::StaticFolder] static_folder: PathBuf,
) -> ShuttlePoise<Data, Error> {
    // Get the discord token set in `Secrets.toml`
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![hello(), chuck()],
            event_handler: |_ctx, event, _framework, _data| {
                Box::pin(event_handler(_ctx, event, _framework, _data))
            },
            ..Default::default()
        })
        .token(discord_token)
        .intents(serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    static_folder: static_folder,
                    api_link: secret_store.get("API_LINK").context("'API_LINK' was not found")?,
                    api_key: secret_store.get("API_KEY").context("'API_KEY' was not found")?,
                })
            })
        })
        .build()
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(framework.into())
}



async fn event_handler(
    ctx: &serenity::Context,
    event: &Event<'_>,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        Event::Ready { data_about_bot } => {
            println!("Logged in as {}", data_about_bot.user.name);
        }
        Event::Message { new_message } => {
            // if bot mentioned
            if new_message.mentions_me(ctx).await? {
                let response = get_clevreq(&new_message.content).await?;

                new_message
                    .reply(ctx, response)
                    .await?;
            }
        }
        _ => {}
    }
    Ok(())
}
