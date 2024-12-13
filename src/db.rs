use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;
use std::env;
use dotenv::dotenv;

pub async fn init_db() -> Result<MySqlPool, sqlx::Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    let pool = MySqlPoolOptions::new()
        .max_connections(5) // Set the max connections to the database
        .connect(&database_url)
        .await?;

    Ok(pool)
}

