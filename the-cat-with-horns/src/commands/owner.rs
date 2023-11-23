//! Commands that can only be invoked by the bot's owner


use poise::serenity_prelude::{CreateEmbed, Color, CreateEmbedAuthor};
use sqlx;
use sqlx_core::query_as::query_as;
use sqlx_postgres::PgPool;
use crate::{Context, Error, helpers::my_embeds::send_embed};



#[derive(Debug ,sqlx::FromRow)]
struct PopeQuote {
    id: i32,
    pl: String,
    en: String
}

impl PopeQuote {
    pub async fn insert_one(pool: &PgPool, pl: &str, en: &str) -> Result<PopeQuote, sqlx::Error> {

        let result = query_as::<_, PopeQuote>(
            "INSERT INTO popequote (pl, en) VALUES ($1, $2) RETURNING *",
            )
            .bind(pl)
            .bind(en)
            .fetch_one(pool)
            .await?;

        Ok(result)
    }

    pub async fn get_all(pool: &PgPool) -> Result<Vec<PopeQuote>, sqlx::Error> {

        let results = query_as::<_, PopeQuote>(
            "SELECT * FROM popequote"
            )
            .fetch_all(pool)
            .await?;

        Ok(results)
    }

    pub async fn get_random(pool: &PgPool) -> Result<PopeQuote, sqlx::Error> {
        let result = query_as::<_, PopeQuote>(
            "SELECT * FROM popequote ORDER BY RANDOM() LIMIT 1
            ")
            .fetch_one(pool)
            .await?;

        Ok(result)
    }
}

#[poise::command(slash_command, prefix_command, owners_only)]
pub async fn popequote_add(
    ctx: Context<'_>,
    #[description = "A popequote in polish"]
    quote_pl: String,
    #[description = "A popequote in english"]
    quote_en: String
) -> Result<(), Error> {

    let quote = PopeQuote::insert_one(&ctx.data().db, &quote_pl, &quote_en).await?;

    ctx.say(format!("Inserted: `{}` - `{}` - `{}`", quote.id, quote.pl, quote.en)).await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command, owners_only)]
pub async fn popequote_all(ctx: Context<'_>) -> Result<(), Error> {

    let quotes = PopeQuote::get_all(&ctx.data().db).await?;

    let quotes_str = quotes.iter()
        .map(|quote| format!("`{} - {} - {}`", quote.id, quote.pl, quote.en))
        .collect::<Vec<_>>()
        .join(",\n");

    let mut embed = CreateEmbed::default();

    embed.title("All quotes:")
        .color(Color::BLURPLE)
        .description(quotes_str);


    send_embed(ctx, embed).await?;

    Ok(())
}


#[poise::command(slash_command, prefix_command, owners_only)]
pub async fn popequote_random(ctx: Context<'_>) -> Result<(), Error> {
    let quote = PopeQuote::get_random(&ctx.data().db).await?;

    let mut author = CreateEmbedAuthor::default();
    author.name("John Paul the 2nd")
    .icon_url("https://media.discordapp.net/attachments/1060711805028155453/1060713256576106606/sc2.png?width=390&height=390");

    let mut embed = CreateEmbed::default();
    embed.title("Quote:")
        .description(format!("*{}*", quote.pl))
        .set_author(author)
        .field(
            "Quote translation:", 
            format!("*{}*", quote.en), 
            true)
        .color(Color::BLURPLE);

    send_embed(ctx, embed).await?;

    Ok(())
}


#[poise::command(slash_command, prefix_command)]
pub async fn paginate(ctx: Context<'_>) -> Result<(), Error> {
    let pages = [
        "Content of first page",
        "Content of second page",
        "Content of third page",
        "Content of fourth page",
    ];

    poise::samples::paginate(ctx, &pages).await?;

    Ok(())
}