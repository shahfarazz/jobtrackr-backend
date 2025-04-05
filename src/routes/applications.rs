

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
use crate::db::job_applications::get_job_applications;
use axum::extract::Query;
use serde::Deserialize;
use crate::db::job_applications::get_applications_by_user;
use uuid::Uuid;



/// Route handler for POST /applications
pub async fn create_application(
    State(pool): State<Arc<PgPool>>,            // Get shared DB pool from state
    Json(payload): Json<NewJobApplication>,     // Parse incoming JSON into struct
) -> impl IntoResponse {
    match insert_job_application(&pool, payload).await {
        Ok(app) => {
            println!("📩 New job app inserted: {:?}", app);
            (StatusCode::CREATED, Json(app)).into_response() // ✅ wrapped
        },
        Err(err) => {
            eprintln!("Error inserting: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response() // ✅ same type
        }
    }
}

#[derive(Deserialize)]
pub struct ApplicationQuery {
    user_id: Option<Uuid>,
}

/// Route handler for GET /applications
pub async fn get_applications(
    State(pool): State<Arc<PgPool>>,
    Query(params): Query<ApplicationQuery>,
) -> impl IntoResponse {
    let result = if let Some(uid) = params.user_id {
        get_applications_by_user(&pool, uid).await
    } else {
        get_job_applications(&pool).await
    };

    match result {
        Ok(apps) => (StatusCode::OK, Json(apps)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
    }
}


