#![allow(dead_code)]

use poise::serenity_prelude::Timestamp;
use reqwest;
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {



    Ok(())
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










