use std::env;

use dotenv::dotenv;

use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};

#[group]
#[commands(ping, dog)]
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
    let api_link = "https://dog.ceo/api/breeds/image/random";

    let request = reqwest::get(api_link).await?;

    let json = request.json::<serde_json::Value>().await?;

    let dog_img_link_str = json
        .get("message")
        .and_then(|link| link.as_str())
        .unwrap_or("No link found...");// Get the string value if it exists


    msg.reply(ctx, dog_img_link_str).await?;

    Ok(())
}

