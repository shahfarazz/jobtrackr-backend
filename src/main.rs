mod config;
mod db;
mod models;
mod routes;

use axum::{Router, routing::post, routing::get};
use std::net::SocketAddr;
use config::Config;
use db::connect_to_db;
use routes::applications::{create_application, get_applications};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let cfg = Config::from_env();

    // Connect to the database
    let db: Arc<sqlx::PgPool> = Arc::new(connect_to_db(&cfg.database_url).await);


    // Build the app router
    let app = Router::new()
        .route("/applications", post(create_application).get(get_applications))
        .route("/", get(root)) // ← add this line
        .with_state(db.clone()); // Share DB pool across routes

    let addr = SocketAddr::from(([127, 0, 0, 1], cfg.port));
    println!("🚀 Server running at http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

// The handler for "/"
async fn root() -> &'static str {
    "JobTrackr backend is alive!"
}
