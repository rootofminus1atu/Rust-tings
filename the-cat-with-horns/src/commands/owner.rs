//! Commands that can only be invoked by the bot's owner

use poise::serenity_prelude::{CreateEmbed, Color};
use crate::{Context, Error, helpers::discord::paginate_str_pages};


#[poise::command(prefix_command, owners_only, hide_in_help)]
pub async fn kill(ctx: Context<'_>) -> Result<(), Error> {
    let _ = ctx.say("Roses are red, I'm going to bed").await;

    ctx.framework()
        .shard_manager()
        .lock()
        .await
        .shutdown_all()
        .await;
    Ok(())
}


#[poise::command(slash_command, prefix_command, hide_in_help)]
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