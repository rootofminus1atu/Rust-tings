use crate::{Context, Error};
use poise::serenity_prelude::{self as serenity, Color};
use serenity::{CreateEmbed, CreateEmbedAuthor};
use crate::helpers::popequotestruct::PopeQuote;
use crate::helpers::discord::send_embed;

async fn fetch_animal_img(url: &str, field_name: &str) -> Result<String, Error> {
    let img: String = reqwest::get(url)
        .await?
        .json::<serde_json::Value>()
        .await?
        .get(field_name)
        .and_then(|value| value.as_str())
        .ok_or("Field not found or not a string")?
        .to_string();

    Ok(img)
}

/// Get a random fluffy fox!
#[poise::command(slash_command, category = "Randomizer")]
pub async fn fox(ctx: Context<'_>) -> Result<(), Error> {
    let url = "https://randomfox.ca/floof";
    let img = fetch_animal_img(url, "image").await?;
    
    let mut embed = CreateEmbed::default();
    embed.title("Here's your fox!")
    .color((255, 0, 0))
    .image(img);

    send_embed(ctx, embed).await?;

    Ok(())
}

/// Pope John Paul the 2nd's wisdom
#[poise::command(slash_command, prefix_command, category = "Randomizer")]
pub async fn popequote(ctx: Context<'_>) -> Result<(), Error> {
    let q = PopeQuote::get_random_quote().ok_or("No quote found somehow...")?;

    let mut author = CreateEmbedAuthor::default();
    author.name("John Paul the 2nd")
    .icon_url("https://media.discordapp.net/attachments/1060711805028155453/1060713256576106606/sc2.png?width=390&height=390");

    let mut embed = CreateEmbed::default();
    embed.title("Quote:")
    .description(format!("*{}*", q.quote_pl))
    .set_author(author)
    .field(
        "Quote translation:", 
        format!("*{}*", q.quote_en), 
        true)
    .color(Color::BLURPLE);

    send_embed(ctx, embed).await?;

    Ok(())
}


