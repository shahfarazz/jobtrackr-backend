mod config;
mod db;
mod models;
mod routes;
mod auth;

use tower_http::cors::CorsLayer;


use axum::{Router, routing::post, routing::get};
use std::net::SocketAddr;
use config::Config;
use db::connect_to_db;
use routes::applications::{create_application, get_applications};
use std::sync::Arc;
use routes::auth_routes::{register_user, login_user};
use axum::middleware;
use crate::auth::middleware::require_auth;
use axum::http::{HeaderValue, Method, HeaderName};



#[tokio::main]
async fn main() {
    let cfg = Config::from_env();

    // Connect to the database
    let db: Arc<sqlx::PgPool> = Arc::new(connect_to_db(&cfg.database_url).await);


    // Set up CORS
    let cors = CorsLayer::new()
    .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())  // ✅ React dev server
    .allow_methods([Method::GET, Method::POST])                             // ✅ Only what's needed
    .allow_headers([HeaderName::from_static("content-type"), HeaderName::from_static("authorization")]);


    let protected_routes = Router::new()
    .route("/", post(create_application).get(get_applications))
    .layer(middleware::from_fn(require_auth));


    let app_routes = Router::new()
    .route("/", get(root))
    .route("/auth/register", post(register_user))
    .route("/auth/login", post(login_user))
    .nest("/applications", protected_routes)
    .with_state(db.clone()); // 👈 apply state here



    // Build the app router
    let app = app_routes
    .layer(cors)
    .with_state(db.clone()); // 👈 this comes AFTER .layer(cors)

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
