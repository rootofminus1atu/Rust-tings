use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use tracing::{info, warn};

pub mod models {
    pub mod customer;
    pub mod product;
}

use models::{customer::Customer, product};

type TypicalRes = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> TypicalRes {
    dotenv().ok();
    tracing_subscriber::fmt().init();

    let database_url = std::env::var("DATABASE_URL")
        .expect("No database url found in environment variables");

    let db = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Could not connect");

    if let Err(err) = sqlx::migrate!().run(&db).await {
        panic!("Migration failed: {}", err);
    }

    println!("Hello, world!");
    //let new_c = Customer::insert_one(&db, "Bo".into(), "Sinn".into(), "rompompom@gmail.com".into(), Some(2000.into())).await?;
    //info!("{:?}", new_c);

    Ok(())
}
