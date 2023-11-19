use anyhow::Context as _;

use poise::serenity_prelude as serenity;
use shuttle_poise::ShuttlePoise;
use shuttle_secrets::SecretStore;

mod helpers {
    pub mod popequotestruct;
    pub mod my_embeds;
    pub mod datetime;
    pub mod misc;
}

mod commands {
    pub mod fun;
    pub mod randomizer;
    pub mod info;
    pub mod admin;
    pub mod events;
}
use commands::fun::{hello, oracle, kazakhstan, sashley, bite};
use commands::randomizer::{fox, popequote};
use commands::info::{botinfo, serverinfo, help};
use commands::admin::{say, kill};
use commands::events::event_handler;


pub struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;



// TODO:
// palette
// translator
// 2137 notification
// automod
// warns
// add popequote
// ocs display
// cleverbot chat
// clairvoyance random message
// change status every few minutes
// calculator maybe?
// improve owners/devs/testers
// help command




#[shuttle_runtime::main]
async fn poise(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> ShuttlePoise<Data, Error> {
    // Get the discord token set in `Secrets.toml`
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                hello(),
                oracle(),
                sashley(),
                kazakhstan(),
                bite(),

                fox(),
                popequote(),

                botinfo(),
                serverinfo(),
                help(),

                say(),
                kill()
                ],
            event_handler: |_ctx, event, _framework, _data| {
                Box::pin(event_handler(_ctx, event, _framework, _data))
            },
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".into()),
                ..Default::default()
            },
            ..Default::default()
        })
        .token(discord_token)
        .intents(
            serenity::GatewayIntents::non_privileged() 
            | serenity::GatewayIntents::MESSAGE_CONTENT
            | serenity::GatewayIntents::GUILD_MEMBERS
            | serenity::GatewayIntents::GUILD_PRESENCES
        )
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build()
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(framework.into())

}








