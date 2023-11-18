use std::collections::HashMap;

use crate::{Context, Error};
use poise::serenity_prelude::{self as serenity, Color, User, OnlineStatus, Channel, ChannelType, ChannelId, GuildChannel};
use serenity::{CreateEmbed, CreateEmbedFooter};
use crate::helpers::my_embeds::send_embed;
use crate::helpers::datetime::pretty_date;

/// Information about the bot!
#[poise::command(slash_command)]
pub async fn botinfo(ctx: Context<'_>) -> Result<(), Error> {
    let me = ctx.serenity_context().cache.current_user();
    let me_user = User::from(me);

    // use this later probably
    let _owners = &ctx.framework().options().owners;

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

    let mut footer = CreateEmbedFooter::default();
    footer.text(format!("Creation date: {}", pretty_date(&g.id.created_at())));

    let mut embed = CreateEmbed::default();
    embed.title("Server information")
        .description(format!("
            🔹**Name:** {}
            🔹**Id:** {}
            🔹**Owner:** {}"
            , g.name, g.id, owner.name))
        .color(Color::BLURPLE)
        .field(
            "Members:", format!("
            🔹**All:** {}
            🔹**Online:** {}"
            , g.member_count, g.members_with_status(OnlineStatus::Online).len()), 
            true)
        .field(
            "Channels: ", 
            format!("
            🔹**Text:** {}
            🔹**Voice:** {}
            ", 
            filter_channels_by_type(&g.channels, ChannelType::Text).len(), 
            filter_channels_by_type(&g.channels, ChannelType::Voice).len()),
            true)
        .set_footer(footer);

    if let Some(icon) = g.icon_url() {
        embed.thumbnail(icon);
    }
    

    send_embed(ctx, embed).await?;
                    
    Ok(())
}



fn filter_channels_by_type(channels: &HashMap<ChannelId, Channel>, channel_type: ChannelType) -> Vec<&GuildChannel> {
    channels
        .iter()
        .filter_map(|(_, channel)| {
            if let Channel::Guild(guild_channel) = channel {
                if guild_channel.kind == channel_type {
                    return Some(guild_channel);
                }
            }
            None
        })
        .collect()
}