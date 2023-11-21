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
use commands::admin::{say, kill, test};
use commands::events::event_handler;

use sqlx_postgres::{PgPool, PgPoolOptions};


// User data, which is stored and accessible in all command invocations
pub struct Data {
    db: PgPool
} 
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

// URGENT:
// fix the self-referential mention in BITE command




#[shuttle_runtime::main]
async fn poise(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> ShuttlePoise<Data, Error> {
    // Get the discord token set in `Secrets.toml`
    let discord_token = secret_store
        .get("DISCORD_BOT_TOKEN")
        .context("'DISCORD_BOT_TOKEN' was not found")?;

    let database_url = secret_store
        .get("DATABASE_URL")
        .context("No database url found in environment variables")?;

        
    let db = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    /*
    sqlx::migrate!()
        .run(&db)
        .await
        .expect("Unable to apply migrations!");
    */

    let data = Data { db };

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
                kill(),
                test()
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
                Ok(data)
            })
        })
        .build()
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(framework.into())

}

