

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json, // ✅ THIS fixes all the `Json` errors
};


use std::sync::Arc;
use crate::models::job_application::{NewJobApplication, JobApplication};
use crate::db::job_applications::insert_job_application;
use sqlx::PgPool;

/// Route handler for POST /applications
pub async fn create_application(
    State(pool): State<Arc<PgPool>>,            // Get shared DB pool from state
    Json(payload): Json<NewJobApplication>,     // Parse incoming JSON into struct
) -> impl IntoResponse {
    match insert_job_application(&pool, payload).await {
        Ok(app) => (StatusCode::OK, Json(app)).into_response(), // ✅ wrapped
        Err(err) => {
            eprintln!("Error inserting: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response() // ✅ same type
        }
    }
}
