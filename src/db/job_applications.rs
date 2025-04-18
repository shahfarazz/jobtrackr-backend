use sqlx::PgPool;
use crate::models::job_application::{JobApplication, NewJobApplication};
use uuid::Uuid;

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
/// Retrieves all job applications from the DB
pub async fn get_job_applications(pool: &PgPool) -> Result<Vec<JobApplication>, sqlx::Error> {
    let apps = sqlx::query_as!(
        JobApplication,
        r#"
        SELECT * FROM job_applications
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(apps)
}

/// gets a job application by its ID
pub async fn get_applications_by_user(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<JobApplication>, sqlx::Error> {
    let apps = sqlx::query_as!(
        JobApplication,
        r#"
        SELECT * FROM job_applications
        WHERE user_id = $1
        ORDER BY applied_at DESC
        "#,
        user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(apps)
}