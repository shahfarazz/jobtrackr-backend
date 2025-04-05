use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDateTime;


/// Struct for reading rows from the DB
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct JobApplication {
    pub id: Uuid,
    pub user_id: Uuid,
    pub company: String,
    pub position: String,
    pub status: String,
    pub applied_at: NaiveDateTime,
    pub notes: Option<String>,
}

/// Struct for receiving JSON from POST request
#[derive(Debug, Deserialize)]
pub struct NewJobApplication {
    pub user_id: Uuid,
    pub company: String,
    pub position: String,
    pub notes: Option<String>,
}
