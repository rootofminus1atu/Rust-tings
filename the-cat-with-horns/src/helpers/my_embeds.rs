use crate::{Context, Error};
use poise::serenity_prelude::{self as serenity};
use serenity::CreateEmbed;

pub async fn send_embed(ctx: Context<'_>, embed: CreateEmbed) -> Result<(), Error> {
    ctx.send(|f: &mut poise::CreateReply<'_>| f
        .embed(|e: &mut CreateEmbed| {*e = embed; e})
    ).await?;

    Ok(())
}