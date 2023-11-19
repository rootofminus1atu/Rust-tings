use std::collections::HashMap;

use poise::serenity_prelude::{Channel, ChannelId, ChannelType, GuildChannel};
use rand::seq::SliceRandom;
use rand;

pub fn random_choice<'a, T>(items: &'a [T]) -> Option<&'a T> {
    let mut rng = rand::thread_rng();
    items.choose(&mut rng)

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