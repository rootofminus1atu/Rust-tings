use std::collections::HashMap;

use chrono::{NaiveDate, Duration};
use poise::serenity_prelude::{Channel, ChannelId, ChannelType, GuildChannel};
use rand::seq::SliceRandom;
use rand;
use rand::Rng;

pub fn random_choice<'a, T>(items: &'a [T]) -> Option<&'a T> {
    let mut rng = rand::thread_rng();
    items.choose(&mut rng)
}

pub fn random_int(lower_bound: i32, upper_bound: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(lower_bound..=upper_bound)
    
}


pub fn random_date(start: NaiveDate, end: NaiveDate) -> NaiveDate {
    let mut rng = rand::thread_rng();

    let days = (end - start).num_days();
    let random_days = rng.gen_range(0..=days); 
    
    start + Duration::days(random_days)
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