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

use tokio_cron_scheduler::{Job, JobScheduler};
use chrono::{DateTime, Utc, Timelike};
use chrono_tz::Europe::Warsaw;

#[derive(Debug, sqlx::FromRow)]
struct Oc {
    id: i32,
    name: String,
    emoji: String,
    short_desc: String,
    long_desc: String,
    created_by: String,
    created_on: String,
    image: String
}

impl Oc {
    pub fn new(name: String, emoji: String, short_desc: String, long_desc: String, created_by: String, created_on: String, image: String) -> Self {
        Oc { id: 0, name, emoji, short_desc, long_desc, created_by, created_on, image }
    }

    pub async fn insert_one(pool: &PgPool, oc: Self) -> Result<Self, sqlx::Error> {
        let result = query_as::<_, Oc>(
            "INSERT INTO oc (name, emoji, short_desc, long_desc, created_by, created_on, image)
            VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *"
            )
            .bind(oc.name)
            .bind(oc.emoji)
            .bind(oc.short_desc)
            .bind(oc.long_desc)
            .bind(oc.created_by)
            .bind(oc.created_on)
            .bind(oc.image)
            .fetch_one(pool)
            .await?;

        Ok(result)
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {


    
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

    sched.add(Job::new("0 37 * * * *", |_, _| {
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










