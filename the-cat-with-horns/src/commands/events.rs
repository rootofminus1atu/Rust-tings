use crate::helpers::misc::random_choice;
use poise::{serenity_prelude::ChannelId, Event};
use crate::{Error, Data};
use poise::serenity_prelude as serenity;
use serenity::model::channel::Message;
use serenity::model::gateway::Activity;
use tokio::time::interval;
use std::time::Duration;
use tokio_cron_scheduler::{Job, JobScheduler};
use chrono::{DateTime, Utc, Timelike};
use chrono_tz::Europe::Warsaw;

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &Event<'_>,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        
        Event::Ready { data_about_bot } => {
            println!("Logged in as {}", data_about_bot.user.name);

            tokio::spawn(change_activity(ctx.clone()));
            

            // scheduling 2137
            let sched = JobScheduler::new().await?;

            let job_ctx = ctx.clone();

            sched.add(Job::new("5 37 * * * *", move |_, _| {
                tokio::task::spawn(send_papiez_msg(job_ctx.clone()));
            })?).await?;

            sched.start().await?;


            /*
            tokio::spawn(say_hi_every_second());
            println!("Repeating background task started");
            
            tokio::spawn(send_periodic_message(ctx.clone()));
            println!("Repeating... another task")
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


async fn send_papiez_msg(ctx: serenity::Context) {
    let now: DateTime<Utc> = Utc::now();
    let now_pl: DateTime<_>  = now.with_timezone(&Warsaw);
    println!("Scheduled task running at: {}", now);
    println!("Poland: {}", now_pl);

    let message_content = "this is a periodic message";

    if now_pl.hour() == 21 {
        println!("REAL PAPIEZ 21");

        let channel_id = ChannelId::from(1152319972320739502);
        // send text msg in that channel
        if let Err(err) = channel_id.say(ctx.http.clone(), message_content).await {
            eprintln!("Failed to send periodic message: {:?}", err);
        }
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
        Activity::watching("The thing"),
        Activity::playing("With fire spells"),
        Activity::watching("Skyrim"),
    ];

    let mut activity_cycle = activities.into_iter().cycle();
    let mut timer = interval(Duration::from_secs(60));

    loop {
        if let Some(activity) = activity_cycle.next() {
            println!("Changing activity to {:?}", activity);
            ctx.set_activity(activity).await;
        }
        println!("Successfully changed activity!");

        timer.tick().await;
    }
}

// Define a function to send a message to the specified channel periodically
/* 

async fn send_periodic_message(ctx: serenity::Context) {
    let channel_id = ChannelId::from(1152319972320739502);  // for now hardcoded

    let mut timer = interval(Duration::from_secs(11));

    loop {
        let message_content = "this is a periodic message";

        if let Err(err) = channel_id.say(ctx.http.clone(), message_content).await {
            eprintln!("Failed to send periodic message: {:?}", err);
        }

        timer.tick().await;
    }
}

async fn say_hi_every_second() {
    let mut timer = interval(Duration::from_secs(10));

    loop {
        println!("Hi!");
        timer.tick().await;
    }
}*/