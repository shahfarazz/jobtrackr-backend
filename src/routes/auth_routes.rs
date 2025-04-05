use axum::{Json, response::IntoResponse, http::StatusCode};
use axum::extract::State;
use serde_json::json;
use std::sync::Arc;
use sqlx::PgPool;

use crate::models::user::{RegisterUser, User};
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::{SaltString, rand_core::OsRng, PasswordHash, PasswordVerifier};
use argon2::password_hash::Error as ArgonError;

pub async fn register_user(
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<RegisterUser>,
) -> impl IntoResponse {
    let RegisterUser { email, password } = payload;

    // Generate salt and hash password
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(h) => h.to_string(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Hashing failed").into_response(),
    };

    // Insert into DB
    let result = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (email, password)
        VALUES ($1, $2)
        RETURNING id, email, password, created_at
        "#,
        email,
        hash
    )
    .fetch_one(&*pool)
    .await;

    match result {
        Ok(user) => (StatusCode::CREATED, Json(json!({ "id": user.id, "email": user.email }))).into_response(),
        Err(err) => {
            eprintln!("Register error: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Register failed").into_response()
        }
    }
}

use crate::models::user::LoginUser;
use jsonwebtoken::{encode, Header, EncodingKey};
use crate::auth::jwt::Claims;

use std::env;

pub async fn login_user(
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<LoginUser>,
) -> impl IntoResponse {
    let LoginUser { email, password } = payload;

    // 1. Fetch user
    let result = sqlx::query!(
        r#"SELECT id, password FROM users WHERE email = $1"#,
        email
    )
    .fetch_optional(&*pool)
    .await;

    let user = match result {
        Ok(Some(user)) => user,
        _ => return (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response(),
    };

    // 2. Verify password
    let parsed_hash = match PasswordHash::new(&user.password) {
        Ok(hash) => hash,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Invalid password hash").into_response(),
    };

    let verify_ok = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();

    if !verify_ok {
        return (StatusCode::UNAUTHORIZED, "Incorrect password").into_response();
    }

    // 3. Generate JWT
    let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "supersecret".into());
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: user.id.to_string(),
        exp: expiration,
    };

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_secret.as_ref()))
        .unwrap();

    (StatusCode::OK, Json(json!({ "token": token }))).into_response()
}
