use poise::serenity_prelude::{CreateEmbed, Color};

use crate::{Context, Error, helpers::{misc::divide_with_strlen, discord::paginate_str_pages}};
use super::{oc::Oc, popequote::PopeQuote};

/// OWNER ONLY
#[poise::command(
    prefix_command,
    slash_command,
    subcommands("popequote", "character"),
    subcommand_required,
    hide_in_help,
    owners_only
)]
pub async fn owner(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(
    prefix_command,
    slash_command,
    subcommands("popequote_add", "popequote_all"),
    subcommand_required
)]
pub async fn popequote(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// (OWNER ONLY) add a popequote
#[poise::command(prefix_command, slash_command, rename = "add")]
pub async fn popequote_add(
    ctx: Context<'_>,
    #[description = "A popequote in polish"]
    quote_pl: String,
    #[description = "A popequote in english"]
    quote_en: String
) -> Result<(), Error> {

    let quote = PopeQuote::insert_one(&ctx.data().db, &quote_pl, &quote_en).await?;

    ctx.say(format!("Inserted: **id**: {}, **pl**: {}, **en**: {}", quote.id, quote.pl, quote.en)).await?;

    Ok(())
}

/// (OWNER ONLY) see all popequotes
#[poise::command(prefix_command, slash_command, rename = "all")]
pub async fn popequote_all(ctx: Context<'_>) -> Result<(), Error> {

    let quotes: Vec<PopeQuote> = PopeQuote::get_all(&ctx.data().db).await?;

    let quotes_strs: Vec<String> = quotes.iter()
        .map(|quote| format!("`id: {} - pl: {} - en: {}`", quote.id, quote.pl, quote.en))
        .collect::<Vec<_>>();

    let pages: Vec<Vec<String>> = divide_with_strlen(quotes_strs, 2000);

    let quote_pages: Vec<String> = pages.iter()
        .map(|v| v.join("\n"))
        .collect::<Vec<_>>();

    let quote_pages_better: &[&str] = &quote_pages.iter()
        .map(|s| s.as_str())
        .collect::<Vec<&str>>()[..];


    println!("{:?}", quote_pages_better);


    let mut embed_base = CreateEmbed::default();

    embed_base.title("All quotes:")
        .color(Color::BLURPLE);

    paginate_str_pages(ctx, quote_pages_better, &embed_base).await?;

    Ok(())
}



#[poise::command(
    prefix_command,
    slash_command,
    subcommands("character_add"),
    subcommand_required
)]
pub async fn character(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// (OWNER ONLY) add a character
#[poise::command(prefix_command, slash_command, rename = "add")]
pub async fn character_add(
    ctx: Context<'_>,
    #[description = "The name of your oc"]
    name: String,
    #[description = "The emoji used as an icon in the SELECT MENU"]
    emoji: String,
    #[description = "The description in the SELECT MENU"]
    short_desc: String,
    #[description = "The description in the EMBED DISPLAY"]
    long_desc: String,
    #[description = "Who made the oc"]
    created_by: String,
    #[description = "When was it created"]
    created_on: String,
    #[description = "The image URL that shows your oc"]
    image: String,
    #[description = "The color of the embed sidebar, HEX ONLY"]
    side_color: String,
) -> Result<(), Error> {
    let new_oc = Oc::new(name, emoji, short_desc, long_desc, created_by, created_on, image, side_color);
    let oc = Oc::insert_one(&ctx.data().db, new_oc).await?;

    ctx.say(format!("Inserted: `id: {}` - `name: {}` - `emoji: {}` - `short_desc: {}` - `long_desc: {}` - `created_by: {}` - `created_on: {}` - `image: {}` - `side_color: {}`",
        oc.id, oc.name, oc.emoji, oc.short_desc, oc.long_desc, oc.created_by, oc.created_on, oc.image, oc.side_color)).await?;

    Ok(())
}
