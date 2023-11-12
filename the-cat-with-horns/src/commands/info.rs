use crate::{Context, Error};
use poise::serenity_prelude::{self as serenity, Color, User};
use serenity::{CreateEmbed, CreateEmbedFooter};
use crate::helpers::my_embeds::send_embed;
use crate::helpers::datetime::pretty_date;

/// Information about the bot!
#[poise::command(slash_command)]
pub async fn botinfo(ctx: Context<'_>) -> Result<(), Error> {
    let me = ctx.serenity_context().cache.current_user();
    let me_user = User::from(me);

    let owners = &ctx.framework().options().owners;

    let mut footer = CreateEmbedFooter::default();
    footer.text(format!("Creation date: {}", pretty_date(&me_user.created_at())));

    let mut embed = CreateEmbed::default();
    embed.title(format!("{}", me_user.name))
        .description("```Cat knows much, tells some. Cat knows many things others do not. Cat wishes you well.```")
        .color(Color::BLURPLE)
        .thumbnail(me_user.face())
        .field("Created by:", "```bot owner```", true)
        .field("Developed by:", "```2 people```", true)
        .field("Tested by:", "```3 people```", true)
        .set_footer(footer);

    send_embed(ctx, embed).await?;

    Ok(())
}

/// Information about the current server!
#[poise::command(slash_command)]
pub async fn serverinfo(ctx: Context<'_>) -> Result<(), Error> {
    let g = ctx.guild().ok_or("I don't think you're in server rn")?;
    let owner = g.owner_id.to_user(&ctx).await?;



    let mut embed = CreateEmbed::default();
    embed.title("Server information")
        .description(format!("
            🔹**Name:** {}
            🔹**Id:** {}
            🔹**Owner:** {}"
            , g.name, g.id, owner.name))
        .color(Color::BLURPLE);

    send_embed(ctx, embed).await?;

    Ok(())
}