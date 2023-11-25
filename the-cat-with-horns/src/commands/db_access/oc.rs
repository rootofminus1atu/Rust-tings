use sqlx_core::query_as::query_as;
use sqlx_postgres::PgPool;



#[derive(Debug, sqlx::FromRow)]
pub struct Oc {
    pub id: i32,
    pub name: String,
    pub emoji: String,
    pub short_desc: String,
    pub long_desc: String,
    pub created_by: String,
    pub created_on: String,
    pub image: String,
    pub side_color: String
}

impl Oc {
    pub fn new(name: String, emoji: String, short_desc: String, long_desc: String, created_by: String, created_on: String, image: String, side_color: String) -> Self {
        Oc { id: 0, name, emoji, short_desc, long_desc, created_by, created_on, image, side_color }
    }

    pub async fn insert_one(pool: &PgPool, oc: Self) -> Result<Self, sqlx::Error> {
        let result = query_as::<_, Oc>(
            "INSERT INTO oc (name, emoji, short_desc, long_desc, created_by, created_on, image, side_color)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *"
            )
            .bind(oc.name)
            .bind(oc.emoji)
            .bind(oc.short_desc)
            .bind(oc.long_desc)
            .bind(oc.created_by)
            .bind(oc.created_on)
            .bind(oc.image)
            .bind(oc.side_color)
            .fetch_one(pool)
            .await?;

        Ok(result)
    }

    pub async fn get_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        let results = query_as::<_, Self>(
            "SELECT * FROM oc"
            )
            .fetch_all(pool)
            .await?;

        Ok(results)
    }
}