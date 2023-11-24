//! Commands that can only be invoked by the bot's owner

use poise::serenity_prelude::{CreateEmbed, Color, CreateEmbedAuthor, CreateSelectMenuOption, ReactionType, CollectComponentInteraction, InteractionResponseType};
use sqlx;
use sqlx_core::query_as::query_as;
use sqlx_postgres::PgPool;
use crate::{Context, Error, helpers::{discord::{send_embed, paginate_str_pages, color_from_hex_str}, misc::divide_with_strlen}};



#[derive(Debug, sqlx::FromRow)]
struct Oc {
    id: i32,
    name: String,
    emoji: String,
    short_desc: String,
    long_desc: String,
    created_by: String,
    created_on: String,
    image: String,
    side_color: String
}

impl Oc {
    pub fn new(name: String, emoji: String, short_desc: String, long_desc: String, created_by: String, created_on: String, image: String, side_color: String) -> Self {
        Oc { id: 0, name, emoji, short_desc, long_desc, created_by, created_on, image, side_color }
    }

    pub async fn insert_one(pool: &PgPool, oc: Self) -> Result<Self, sqlx::Error> {
        let result = query_as::<_, Oc>(
            "INSERT INTO oc (name, emoji, short_desc, long_desc, created_by, created_on, image, side_color)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *"
            )
            .bind(oc.name)
            .bind(oc.emoji)
            .bind(oc.short_desc)
            .bind(oc.long_desc)
            .bind(oc.created_by)
            .bind(oc.created_on)
            .bind(oc.image)
            .bind(oc.side_color)
            .fetch_one(pool)
            .await?;

        Ok(result)
    }

    pub async fn get_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        let results = query_as::<_, Self>(
            "SELECT * FROM oc"
            )
            .fetch_all(pool)
            .await?;

        Ok(results)
    }
}

#[poise::command(slash_command, prefix_command, owners_only)]
pub async fn oc_add(
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


#[poise::command(slash_command, prefix_command)]
pub async fn character(ctx: Context<'_>) -> Result<(), Error> {
    let ocs = Oc::get_all(&ctx.data().db).await?;

    let ctx_id = ctx.id();
    let menu_id = format!("{}menu", ctx_id);

    let menu_options = ocs.iter()
        .map(|oc| {
            let mut mo = CreateSelectMenuOption::default();
            mo.value(&oc.id)
                .label(&oc.name)
                .description(&oc.short_desc)
                .emoji(oc.emoji.clone().parse::<ReactionType>().unwrap());
            mo
        })
        .collect::<Vec<_>>();

    let mut author = CreateEmbedAuthor::default();
    author.name("Character info")
        .icon_url("https://images.emojiterra.com/google/android-11/512px/1f431.png");
    

    ctx.send(|m| {
        m.components(|c| {
            c.create_action_row(|a| {
                a.create_select_menu(|s| {
                    s.options(|o| {
                        o.set_options(menu_options)
                    })
                    .custom_id(&menu_id)
                    .placeholder("Select a character to display!")
                })
            })
        }).embed(|e| {
            e.author(|a| { *a = author.clone(); a })
                .description("Select an OC from below to see their info!")
        })
    })
    .await?;

    while let Some(choice) = CollectComponentInteraction::new(ctx)
        .filter(move |choice| choice.data.custom_id.starts_with(&ctx_id.to_string()))
        .timeout(std::time::Duration::from_secs(60))
        .await
    {
        choice.create_interaction_response(ctx, |r| {
            r.kind(InteractionResponseType::UpdateMessage)
                .interaction_response_data(|d| {
                    d.embed(|e| {
                        let id = choice.data.values[0].clone();

                        let oc = ocs.iter().find(|oc| oc.id.to_string() == id).unwrap();

                        e.author(|a| { *a = author.clone(); a })
                            .title(&oc.name)
                            .description(&oc.long_desc)
                            .field(
                                "Created by:",
                                &oc.created_by,
                                true)
                            .field(
                                "Created on:",
                                &oc.created_on,
                                true)
                            .image(&oc.image)
                            .color(color_from_hex_str(&oc.side_color).unwrap_or(Color::BLURPLE))
                            .footer(|f| {
                                f.text("You can select another OC from the dropdown menu below!")
                            })
                    })
                })
        })
        .await?;
    }
    Ok(())
}




#[derive(Debug, sqlx::FromRow)]
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

    let quotes: Vec<PopeQuote> = PopeQuote::get_all(&ctx.data().db).await?;

    let quotes_strs: Vec<String> = quotes.iter()
        .map(|quote| format!("`id: {} - {} - {}`", quote.id, quote.pl, quote.en))
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
        "apples",
        "oranges and",
        "Content of third page",
        "dates",
    ];

    let mut embed_base = CreateEmbed::default();
    embed_base.title("The stuff")
        .color(Color::BLURPLE);
        // a footer etc


    paginate_str_pages(ctx, &pages, &embed_base).await?;

    Ok(())
}