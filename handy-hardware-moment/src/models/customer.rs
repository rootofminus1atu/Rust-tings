use sqlx::{postgres::types::PgMoney, PgPool};


/// goofy ahh money behaviot, 2000.into() becomes 20.00 euro, keep this in mind for later
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Customer {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub balance: PgMoney
    
}

impl Customer {
    pub async fn insert_one(pool: &PgPool, first_name: String, last_name: String, email: String, balance: Option<PgMoney>) -> Result<Self, sqlx::Error> {
        
        // there's prob a better way to do this but whatever
        let sql = match balance {
            Some(_) => "INSERT INTO customer (first_name, last_name, email, balance) VALUES ($1, $2, $3, $4) RETURNING *",
            None => "INSERT INTO customer (first_name, last_name, email) VALUES ($1, $2, $3) RETURNING *"
        };
        
        let mut query = sqlx::query_as(sql)
            .bind(first_name)
            .bind(last_name)
            .bind(email);

        if let Some(b) = balance {
            query = query.bind(b);
        };
        
        let new_customer: Self = query.fetch_one(pool).await?;

        Ok(new_customer)
    }
}
