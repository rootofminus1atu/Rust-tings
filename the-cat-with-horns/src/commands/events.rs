use crate::helpers::{misc::{random_choice, random_int, random_date}, datetime::pretty_date};
use poise::{serenity_prelude::{ChannelId, Client, GatewayIntents}, Event};
use tracing::debug;
use crate::{Error, Data};
use poise::serenity_prelude as serenity;
use serenity::model::channel::Message;
use serenity::model::gateway::Activity;
use tokio::time::interval;
use std::{time::Duration, sync::Arc};
use chrono::{DateTime, Utc, Timelike};
use chrono_tz::Europe::Warsaw;
use tokio_cron::{Scheduler, Job};




pub async fn event_handler(
    ctx: &serenity::Context,
    event: &Event<'_>,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {

    match event {
        
        Event::Ready { data_about_bot } => {
            println!("Logged in as {}", data_about_bot.user.name);

            println!("Spawning change_activity");
            tokio::spawn(change_activity(ctx.clone()));

            println!("Spawning clairvoyance");
            tokio::spawn(clairvoyance(ctx.clone()));
            

            let mut scheduler = Scheduler::utc();

            let h = "hi".to_string();

            // scheduler.add(Job::new("*/1 * * * * *", simple_async_fn));
            let ctx_clone = ctx.clone();
            scheduler.add(Job::new("*/2 24 * * * *", move || {
                send_papiez_msg(ctx_clone.clone())  // fucking double clone
            }));


            /* 
            let handlerrr = || async {
                let _cloned = ctx.http.clone();
                println!("hi");
            };

            // I WISH THIS WORKED
            
            let every_day = every(1).day()
                .at(16, 25, 00)
                .in_timezone(&Utc)
                .perform(handlerrr);
        
            tokio::spawn(every_day);
            */

        }
        Event::Message { new_message } => {
            // if bot mentioned
            if new_message.mentions_me(ctx).await? {
                random_response(ctx, &new_message).await?;
            }
        }
        _ => {}
    }
    Ok(())
}


async fn clairvoyance(ctx: serenity::Context) {
    let prophecies = vec![
        "The heat death of the universe",
        "2006 HONDA CIVIC"
    ];
    
    loop {
        let channel_id = ChannelId::from(1031977836849922111); 

        let start = Utc::now().naive_utc();
        let years = 100;
        let in_secs = years * 3600 * 24 * 365;
        let end = start + Duration::from_secs(in_secs);

        let date = random_date(start.date(), end.date());
        let prophecy = random_choice(&prophecies).copied().unwrap_or("The heat death of the universe");
        let msg = format!("{}, {}", pretty_date(&date), prophecy);

        if let Err(why) = channel_id.say(ctx.http.clone(), msg).await {
            eprintln!("Failed to send clairvoyance message: {:?}", why);
        }

        let hours = random_int(1, 3);
        let in_secs = hours * 3600;
        println!("Sleeping for {} hours", hours);
        tokio::time::sleep(Duration::from_secs(in_secs as u64)).await;
    }
}


async fn send_papiez_msg(ctx: serenity::Context) {
    let now: DateTime<Utc> = Utc::now();
    let now_pl: DateTime<_>  = now.with_timezone(&Warsaw);

    let message = "2137";
    println!("fake papiez 21");

    if now_pl.hour() == 4 {
        println!("REAL PAPIEZ 21");

        let channel_id = ChannelId::from(1031977836849922111);
        // send text msg in that channel

        let res = channel_id.say(&ctx.http, message).await;
        println!("the {:?}", res);

        // if let Err(why) =  {
        //     eprintln!("Failed to send 2137 message: {:?}", why);
        // }
    }
}




async fn random_response(ctx: &serenity::Context, msg: &Message) -> Result<(), Error> {
    let responses = vec![
        "Who r u",
        "I was mentioned!",
        ":3"
    ];
    
    let response = random_choice(&responses).copied().unwrap_or("Hi");

    msg.reply(ctx, response).await?;

    Ok(())
}

async fn change_activity(ctx: serenity::Context) {
    let activities = vec![
        Activity::watching("dragons"),
        Activity::playing("with fire spells"),
        Activity::watching("over Skyrim"),
    ];

    let mut activity_cycle = activities.into_iter().cycle();
    let mut timer = interval(Duration::from_secs(60));

    loop {
        timer.tick().await;

        if let Some(activity) = activity_cycle.next() {
            ctx.set_activity(activity).await;
        }
    }
}



// Define a function to send a message to the specified channel periodically
/* 

async fn send_periodic_message(ctx: serenity::Context) {
    let channel_id = ChannelId::from(1152319972320739502);  // for now hardcoded

    let mut timer = interval(Duration::from_secs(11));

    loop {
        timer.tick().await;

        let message_content = "this is a periodic message";

        if let Err(err) = channel_id.say(ctx.http.clone(), message_content).await {
            eprintln!("Failed to send periodic message: {:?}", err);
        }
    }
}

async fn say_hi_every_second() {
    let mut timer = interval(Duration::from_secs(10));

    loop {
        println!("Hi!");
        timer.tick().await;
    }
}*/












































































































