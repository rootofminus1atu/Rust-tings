#![allow(dead_code)]

use poise::serenity_prelude::{Timestamp, Color};
use reqwest;
use sqlx_core::query_as::query_as;
use sqlx_postgres::PgPool;
use tokio;
use ordinal::Ordinal;
use rand::prelude::*;
use tokio_postgres::NoTls;
use sqlx::postgres::PgPoolOptions;

#[derive(Debug, sqlx::FromRow)]
struct User {
    id: i32,
    name: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

use tokio_cron_scheduler::{JobScheduler};
use chrono::{DateTime, Utc, Timelike, Duration};
use chrono_tz::Europe::Warsaw;
// use tokio_schedule::{every, Job};

use chrono::prelude::{NaiveDate, NaiveDateTime};

use tokio_cron::{Scheduler, Job};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut scheduler = Scheduler::utc();

    let h = "hi".to_string();

    // scheduler.add(Job::new("*/1 * * * * *", simple_async_fn));
    scheduler.add(Job::new("*/5 24 * * * *", move || {
        async_fn_with_args(h.clone())
    }));

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        println!("{}", chrono::Utc::now());
    }

    Ok(())
}

async fn simple_async_fn() {
    println!("{} Hello, world!", chrono::Utc::now());
}

async fn async_fn_with_args(name: String) {
    println!("{} Hello, world! {}", chrono::Utc::now(), name);
}



async fn tokio_schedule_test_not_working() -> Result<(), Box<dyn std::error::Error>> {
    /* 
    let x = "idk".to_string();

    my_perform(|| async { say_thing(x.clone()).await });


    let every_day = every(1).day()
        .at(16, 05, 00)
        .in_timezone(&Utc)
        .perform(|| async { 
            say_thing(x.clone()).await 
        }); // how

    tokio::spawn(every_day);
    

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        println!("{}", chrono::Utc::now());
    }

    */
    
    Ok(())
}



fn old_cron_scheduler_rest() -> Result<(), Box<dyn std::error::Error>> {
                /* 
            // scheduling 2137
            println!("Creating a new scheduler");
            let mut sched = JobScheduler::new().await?;
            // THIS THING UNCOMMENT LATER sched.shutdown().await?;

            println!("creating job_ctx");
            let job_ctx = ctx.clone();

            println!("Adding to schedule");
            sched.add(Job::new("5 37 * * * *", move |_, _| {
                tokio::task::spawn(send_papiez_msg(job_ctx.clone()));
            })?).await?;

            println!("Starting schedule");
            sched.start().await?;
            */
    
    Ok(())
}

/// working but won't work in the bot... stupid move
async fn tokio_schedule_test() -> Result<(), Box<dyn std::error::Error>> {
    /* 
    let every_day = every(1).day().at(16, 05, 00)
        .in_timezone(&Utc).perform(|| async { println!("I'm scheduled!") });
    tokio::spawn(every_day);
    

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        println!("{}", chrono::Utc::now());
    }*/

    Ok(())
}


fn tokio_interval_test() -> Result<(), Box<dyn std::error::Error>> {

    /*
    let now = chrono::Utc::now().naive_utc();
    let desired_time = now.date().and_hms_opt(21, 37, 0).unwrap();

    // start with a safe check for overtime
    let start = if now > desired_time {
        let tomorrow = now + Duration::days(1);
        tomorrow.date().and_hms_opt(21, 37, 0).unwrap() 
    } else {
        desired_time
    };


    let period = chrono::Duration::days(1).to_std().unwrap();

    let mut interval = tokio::time::interval_at(tokio::time::Instant::now() + start, period);

    loop {
        interval.tick().await;
    }
     */

    Ok(())
}

fn hex_color_test() -> Result<(), Box<dyn std::error::Error>> {
    let hex_string = "0x123abc";

    println!("{:?}", color_from_hex_str(hex_string).unwrap().hex());

    Ok(())
}

use regex::Regex;

fn is_valid_hex_color(input: &str) -> bool {
    let hex_color_regex = Regex::new(r#"((0x)|(#))?[\dA-Fa-f]{6}"#).unwrap();
    hex_color_regex.is_match(input)
}

fn color_from_hex_str(input: &str) -> Result<Color, Box<dyn std::error::Error>> {
    if !is_valid_hex_color(input) {
        "how tf do I return a ParseIntError saying that a hex value was incorrect, well it was incorrect so please check your hex value".parse::<u32>()?;
    }

    let trimmed_input = input.trim_start_matches(|c| c == '#' || c == '0').trim_start_matches("x");

    let color = u32::from_str_radix(trimmed_input, 16)
        .map(|hex_value| Color::from(hex_value))?;

    Ok(color)
}

fn pagination_test() -> Result<(), Box<dyn std::error::Error>> {
    let input_list = [
        "apples",
        "oranges and",
        "Content of third page",
        "dates",
    ];    
    let str_limit = 20;

    let result = divide_with_strlen(&input_list, str_limit);

    for (i, page) in result.iter().enumerate() {
        println!("Page {}: {:?}", i + 1, page);
    }

    Ok(())
}


fn divide_with_strlen<'a>(list: &'a [&'a str], str_limit: i32) -> Vec<Vec<&'a str>> {
    // Implementation goes here
    // You can use tokio::task::spawn or tokio::task::spawn_blocking for parallelism if needed
    let mut result = Vec::new();
    let mut current_page = Vec::new();
    let mut current_length = 0;

    for &item in list {
        let item_length = item.len() as i32;
        if current_length + item_length > str_limit {
            result.push(std::mem::take(&mut current_page));
            current_length = 0;
        }

        current_page.push(item);
        current_length += item_length;
    }

    if !current_page.is_empty() {
        result.push(current_page);
    }

    result
}

async fn cron_test() -> Result<(), Box<dyn std::error::Error>> {
    let sched = JobScheduler::new().await?;

    sched.add(tokio_cron_scheduler::Job::new("0 37 * * * *", |_, _| {
        let now: DateTime<Utc> = Utc::now();
        let now_pl: DateTime<_>  = now.with_timezone(&Warsaw);
        println!("Scheduled task running at: {}", now);
        println!("Poland: {}", now_pl);

        if now_pl.hour() == 18 {
            println!("18")
        }

        if now_pl.hour() == 19 {
            println!("PAPIEZ 19")
        }

        if now_pl.hour() == 20 {
            println!("20")
        }

    })?).await?;

    sched.start().await?;

    // Wait a while so that the jobs actually run
    println!("before sleep");
    tokio::time::sleep(core::time::Duration::from_secs(100)).await;
    println!("after sleep");


    Ok(())
}

async fn sqlx_text_2() -> Result<(), Box<dyn std::error::Error>> {
    // Define the database URL
    let database_url = "db";

    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    println!("connect done");

    // Execute the insert query
    let inserted_user: User = sqlx::query_as!(
        User,
        "INSERT INTO test_table2 (name) VALUES ($1) RETURNING *",
        "someone else",
    )
    .fetch_one(&pool)
    .await?;

    println!("New user inserted successfully: {:?}", inserted_user);

    Ok(())
}

async fn sqlx_test() -> Result<(), Box<dyn std::error::Error>> {

    #[derive(Debug, sqlx::FromRow)]
    struct TestTableRow {
        name: String,
        // Add other fields as needed based on your table columns
    }

    let database_url = "db";

    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    // Execute a simple query
    let rows: Vec<TestTableRow> = sqlx::query_as("SELECT name FROM test_table")
        .fetch_all(&pool)
        .await?;

    // Process the results
    for row in rows {
        println!("Name: {}", row.name);
        // Access other fields if present in the struct
    }
    
    Ok(())
}

async fn tokio_postgres_test() -> Result<(), Box<dyn std::error::Error>> {
    let connection_str = "db";

    // Connect to the database
    let (client, connection) =
        tokio_postgres::connect(connection_str, NoTls).await?;

    // Spawn a task to process the connection in the background
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Execute a simple query
    for row in client.query("SELECT name FROM test_table", &[]).await? {
        let name: &str = row.get("name");
        println!("Name: {}", name);
    }

    Ok(())
}

fn random_choice_test() -> Result<(), Box<dyn std::error::Error>> {
    let strings = vec!["one", "two", "three"];
    if let Some(random_string) = random_choice(&strings) {
        println!("{:?}", random_string);
    }

    let numbers = vec![1, 2, 3];
    if let Some(random_number) = random_choice(&numbers) {
        println!("{:?}", random_number);
    }

    Ok(())
}

fn random_choice<'a, T>(items: &'a [T]) -> Option<&'a T> {
    let mut rng = rand::thread_rng();
    items.choose(&mut rng)
}

async fn date_test() -> Result<(), Box<dyn std::error::Error>> {
    let ts = Timestamp::now();
    
    let day = ts.format("%d").to_string().parse::<i32>();
    let day_ordinal = Ordinal(day.unwrap()).to_string();
    let month = ts.format("%B");
    let year = ts.format("%Y");

    println!("{} {} {}", month, day_ordinal, year);

    Ok(())
}

async fn animal_test() -> Result<(), Box<dyn std::error::Error>> {
    let img = fetch_animal_img("https://randomfox.ca/floof", "image").await?;

    println!("{:?}", img);

    Ok(())
}

async fn fetch_animal_img(url: &str, field_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let img: String = reqwest::get(url)
        .await?
        .json::<serde_json::Value>()
        .await?
        .get(field_name)
        .and_then(|value| value.as_str())
        .ok_or("Field not found or not a string")?
        .to_string();

    Ok(img)
}










