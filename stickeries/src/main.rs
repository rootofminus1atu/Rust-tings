use teloxide::{prelude::*, types::Sticker, net::Download};
use tokio::{fs::File, io::AsyncWriteExt};
use std::io::Cursor;
use dotenvy::dotenv;
use tempfile::NamedTempFile;
use std::io::Write;
use poise::serenity_prelude as serenity;


struct TelegramBot {
    bot: teloxide::Bot
}

impl TelegramBot {
    pub fn new(token: &str) -> Self {
        let bot = Bot::new(token);
        Self { bot }
    }
    
    async fn steal_stickers(&self, sticker_set_name: &str) -> Result<Vec<NamedTempFile>, Error> {
        println!("Fetching sticker set: {}", sticker_set_name);
        let sticker_set = self.bot.get_sticker_set(sticker_set_name).send().await.unwrap();

        let mut handles = vec![];
        let mut temp_files = vec![];

        for (i, sticker) in sticker_set.stickers.iter().take(5).enumerate() {
            let bot = self.bot.clone();
            let sticker = sticker.clone();
            let handle = tokio::spawn(async move {
                download_sticker(&bot, &sticker, i).await
            });
            handles.push(handle);
        }

        for handle in handles {
            match handle.await.unwrap() {
                Ok(temp_file) => temp_files.push(temp_file),
                Err(e) => eprintln!("failed to download sticker: {}", e),
            }
        }

        Ok(temp_files)
    }
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, (), Error>;
struct Data {}

struct DiscordBot {
    bot: serenity::Client
}

impl DiscordBot {
    pub async fn new(token: &str) -> Self {
        let framework = poise::Framework::<Data, Error>::builder()
            .options(poise::FrameworkOptions {
                commands: vec![],
                ..Default::default()
            })
            .setup(|ctx, _ready, framework| {
                Box::pin(async move {
                    poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                    Ok(Data {})
                })
            })
            .build();
    
        let client = serenity::ClientBuilder::new(token, serenity::GatewayIntents::non_privileged())
            .framework(framework)
            .await
            .unwrap();

        Self { bot: client }
    }
}


struct App {
    telegram_bot: TelegramBot,
    discord_bot: DiscordBot
}

impl App {
    pub async fn new(telegram_token: &str, discord_token: &str) -> Self {
        Self {
            telegram_bot: TelegramBot::new(telegram_token),
            discord_bot: DiscordBot::new(discord_token).await
        }
    }

    pub async fn start(&mut self) {

    }

}

// TODO: 
// handle animated stickers (make it a .apng or .gif, .apng for stickers only)



#[tokio::main]
async fn main() {
    dotenv().ok();

    let discord_token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let telegram_token = std::env::var("TELEGRAM_TOKEN").expect("missing TELEGRAM_TOKEN");
    let sticker_set_name = "TFortress2";
    

}

async fn upload_emojis(ctx: Context<'_>) -> Result<(), Error> {

    Ok(())
}


async fn download_sticker(bot: &Bot, sticker: &Sticker, index: usize) -> Result<NamedTempFile, Error> {
    let file_info = bot.get_file(&sticker.thumbnail.as_ref().map(|f| f.file.id.clone()).unwrap_or(sticker.file.id.clone())).send().await?;
    
    let mut file_data = Vec::new();
    bot.download_file(&file_info.path, &mut Cursor::new(&mut file_data)).await?;

    // let file_path = format!("res/sticker_{}.png", index);
    // let mut file = File::create(&file_path).await?;
    // file.write_all(&file_data).await?;

    let mut temp_file = NamedTempFile::new()?;
    temp_file.write_all(&file_data)?;

    println!("downloaded to temp file: {:?}", temp_file.path());
    Ok(temp_file)
}
