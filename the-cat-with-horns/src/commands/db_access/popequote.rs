use sqlx_core::query_as::query_as;
use sqlx_postgres::PgPool;

#[derive(Debug, sqlx::FromRow)]
pub struct PopeQuote {
    pub id: i32,
    pub pl: String,
    pub en: String
}

impl PopeQuote {
    pub async fn insert_one(pool: &PgPool, pl: &str, en: &str) -> Result<PopeQuote, sqlx::Error> {

        let result = query_as::<_, PopeQuote>(
            "INSERT INTO popequote (pl, en) VALUES ($1, $2) RETURNING *",
            )
            .bind(pl)
            .bind(en)
            .fetch_one(pool)
            .await?;

        Ok(result)
    }

    pub async fn get_all(pool: &PgPool) -> Result<Vec<PopeQuote>, sqlx::Error> {

        let results = query_as::<_, PopeQuote>(
            "SELECT * FROM popequote"
            )
            .fetch_all(pool)
            .await?;

        Ok(results)
    }

    pub async fn get_random(pool: &PgPool) -> Result<PopeQuote, sqlx::Error> {
        let result = query_as::<_, PopeQuote>(
            "SELECT * FROM popequote ORDER BY RANDOM() LIMIT 1
            ")
            .fetch_one(pool)
            .await?;

        Ok(result)
    }
}