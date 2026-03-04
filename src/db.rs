use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

pub async fn conn() -> sqlx::Pool<sqlx::Postgres> {
    dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL is required");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(url.as_str())
        .await
        .expect("failed to connect to the db")
}
