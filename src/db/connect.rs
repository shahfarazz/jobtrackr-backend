use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn connect_to_db(database_url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Failed to connect to DB")
}
