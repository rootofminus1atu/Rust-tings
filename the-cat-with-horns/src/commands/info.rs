use crate::{Context, Error};
use poise::serenity_prelude::futures::future::join_all;
use poise::serenity_prelude::{self as serenity, Color, User, OnlineStatus, ChannelType, CollectComponentInteraction, InteractionResponseType, CreateSelectMenu, CreateEmbedAuthor, ReactionType, CreateSelectMenuOption};
use serenity::{CreateEmbed, CreateEmbedFooter};
use crate::helpers::discord::{send_embed, color_from_hex_str};
use crate::helpers::datetime::pretty_date;
use crate::helpers::discord::filter_channels_by_type;
use crate::commands::db_access::oc::Oc;

/// Information about the bot!
#[poise::command(slash_command)]
pub async fn botinfo(ctx: Context<'_>) -> Result<(), Error> {
    let me = ctx.serenity_context().cache.current_user();
    let me_user = User::from(me);

    // in the future improve the owners categorization and querying

    let owner_futures: Vec<_> = ctx.framework()
        .options() 
        .owners
        .iter()
        .map(|owner_id| owner_id.to_user(ctx))
        .collect();
        
    let owners: Vec<Result<User, serenity::prelude::SerenityError>> = join_all(owner_futures).await;

    let owners_string: String = owners.into_iter()
        .filter_map(Result::ok)
        .map(|user| user.name)
        .collect::<Vec<_>>()
        .join(", ");




    let mut footer = CreateEmbedFooter::default();
    footer.text(format!("Creation date: {}", pretty_date(&me_user.created_at().date_naive())));

    let mut embed = CreateEmbed::default();
    embed.title(format!("{}", me_user.name))
        .description("```Cat knows much, tells some. Cat knows many things others do not. Cat wishes you well.```")
        .color(Color::BLURPLE)
        .thumbnail(me_user.face())
        .field("Created by:", format!("```{}```", owners_string), true)
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
    footer.text(format!("Creation date: {}", pretty_date(&g.id.created_at().date_naive())));


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

    let mut menu = CreateSelectMenu::default();
    menu.custom_id(&menu_id)
        .options(|o| o.set_options(menu_options.clone()))
        .placeholder("Select a character to display!");


    let mut message = ctx.send(|m| {
        m.components(|c| {
            c.create_action_row(|a| {
                a.create_select_menu(|s| {
                    *s = menu.clone();
                    s
                })
            })
        }).embed(|e| {
            e.author(|a| { *a = author.clone(); a })
                .description("Select an OC from below to see their info!")
        })
    })
    .await?
    .into_message()
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

    message.edit(ctx, |m| {
        m.components(|c| {
            c.create_action_row(|a| {
                a.create_select_menu(|s| {
                    *s = menu.clone();
                    s.disabled(true)
                })
            })
        })
    }).await?;

    Ok(())
}



#[poise::command(slash_command, prefix_command)]
pub async fn help(ctx: Context<'_>, command: Option<String>) -> Result<(), Error> {
    let configuration = poise::builtins::HelpConfiguration {
        // [configure aspects about the help message here]
        ..Default::default()
    };
    poise::builtins::help(ctx, command.as_deref(), configuration).await?;
    Ok(())
}