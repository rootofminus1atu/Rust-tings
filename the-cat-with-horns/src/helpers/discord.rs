use std::collections::HashMap;
use regex::Regex;

use crate::{Context, Error};
use poise::serenity_prelude::{self as serenity, InteractionResponseType, ChannelId, Channel, ChannelType, GuildChannel, Color};
use serenity::CreateEmbed;

fn is_valid_hex_color(input: &str) -> bool {
    let hex_color_regex = Regex::new(r#"((0x)|(#))?[\dA-Fa-f]{6}"#).unwrap();
    hex_color_regex.is_match(input)
}

pub fn color_from_hex_str(input: &str) -> Result<Color, Error> {
    if !is_valid_hex_color(input) {
        Err("The hex value found didn't have enough digits (yes 6 is a requirement, # and 0x prefixes are allowed though)")?;
    }

    let trimmed_input = input.trim_start_matches(|c| c == '#' || c == '0').trim_start_matches("x");

    let color = u32::from_str_radix(trimmed_input, 16)
        .map(|hex_value| Color::from(hex_value))?;

    Ok(color)
}


pub async fn send_embed(ctx: Context<'_>, embed: CreateEmbed) -> Result<(), Error> {
    ctx.send(|f: &mut poise::CreateReply<'_>| f
        .embed(|e: &mut CreateEmbed| {*e = embed; e})
    ).await?;

    Ok(())
}

pub async fn paginate_str_pages(ctx: Context<'_>, pages: &[&str], embed_base: &CreateEmbed) -> Result<(), Error> {
    if pages.len() <= 0 {
        Err("No pages found")?;
    }
    
    if pages.len() == 1 {
        ctx.send(|m| {
            m.embed(|e| {
                *e = embed_base.clone();
                e.description(pages[0]);
                e
            })
        })
        .await?;

        return Ok(())
    }
    
    let ctx_id = ctx.id();
    let prev_button_id = format!("{}prev", ctx_id);
    let next_button_id = format!("{}next", ctx_id);

    let mut current_page = 0;

    ctx.send(|m| {
        m.embed(|e| {
            *e = embed_base.clone(); 
            e.description(pages[current_page]);
            e.footer(|f| {
                f.text(format!("Page {}/{}", current_page + 1, pages.len()))
            });
            e
        })
        .components(|c| {
            c.create_action_row(|a| {
                a
                .create_button(|b| b.custom_id(&prev_button_id).emoji('◀'))
                .create_button(|b| b.custom_id(&next_button_id).emoji('▶'))
            })
        })
    })
    .await?;

    while let Some(press) = serenity::CollectComponentInteraction::new(ctx)
        .filter(move |press| press.data.custom_id.starts_with(&ctx_id.to_string()))
        .timeout(std::time::Duration::from_secs(60))
        .await 
    {
        if press.data.custom_id == next_button_id {
            current_page += 1;
            if current_page >= pages.len() {
                current_page = 0;
            }
        } else if press.data.custom_id == prev_button_id {
            current_page = current_page.checked_sub(1).unwrap_or(pages.len() - 1);
        } else {
            continue;
        }

        press.create_interaction_response(ctx, |i| {
            i.kind(InteractionResponseType::UpdateMessage)
                .interaction_response_data(|m| {
                    m.embed(|e| {
                        *e = embed_base.clone(); 
                        e.description(pages[current_page]);
                        e.footer(|f| {
                            f.text(format!("Page {}/{}", current_page + 1, pages.len()))
                        });
                        e
                    })
                })
        }).await?;
    }

    Ok(())
}



pub fn filter_channels_by_type(channels: &HashMap<ChannelId, Channel>, channel_type: ChannelType) -> Vec<&GuildChannel> {
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