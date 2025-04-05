use sqlx::PgPool;
use crate::models::job_application::{JobApplication, NewJobApplication};

/// Inserts a new job application into the DB and returns it
pub async fn insert_job_application(
    pool: &PgPool,
    new_app: NewJobApplication,
) -> Result<JobApplication, sqlx::Error> {
    let record = sqlx::query_as!(
        JobApplication,
        r#"
        INSERT INTO job_applications (user_id, company, position, notes)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
        new_app.user_id,
        new_app.company,
        new_app.position,
        new_app.notes,
    )
    .fetch_one(pool)
    .await?;

    Ok(record)
}
