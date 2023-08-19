use std::env;

use dotenv::dotenv;

use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: serenity::model::gateway::Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv().ok();
    // Now you can access environment variables using env::var
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not found in .env");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
        // let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::all();
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

#[command]
async fn dog(ctx: &Context, msg: &Message) -> CommandResult {
    // make a request to the dog api
    // the dog api has link "https://dog.ceo/api/breeds/image/random"
    // which returns a json object with a field "message" that contains a link to a random dog image
    let dog_api_link = "https://dog.ceo/api/breeds/image/random";

    let dog_img_link_str = reqwest::get(dog_api_link)
        .await?
        .json::<serde_json::Value>()
        .await?
        .get("message")
        .unwrap()
        .to_string();

    // gotta add serde_json and reqwest to Cargo.toml

    msg.reply(ctx, dog_img_link_str).await?;

    Ok(())
}

