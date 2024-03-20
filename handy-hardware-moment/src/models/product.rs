use sqlx::{postgres::types::PgMoney, prelude::FromRow, PgPool};




#[derive(Default, FromRow)]
pub struct Product {
    pub id: i32,
    pub price: PgMoney,
    pub stock: i32
}

impl Product {
    pub async fn insert_one(pool: &PgPool, price: PgMoney, stock: i32) -> Result<Self, sqlx::Error> {
        let new_product: Self = sqlx::query_as(
            "INSERT INTO product (price, stock) VALUES ($1, $2) RETURNING *"
        )
        .bind(price)
        .bind(stock)
        .fetch_one(pool)
        .await?;

        Ok(new_product)
    }
}