use axum::{
    http::{Request, StatusCode},
    response::Response,
    body::Body,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::auth::jwt::Claims;
use std::env;
use axum::middleware::Next;


pub async fn require_auth(
    mut req: Request<Body>,         // ✅ use Body here
    next: Next,               // ✅ use Body here too
) -> Result<Response, StatusCode> {
    // Extract token from "Authorization: Bearer ..."
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Decode the token
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "supersecret".into());
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Optionally attach user ID
    req.extensions_mut().insert(token_data.claims.sub.clone());

    Ok(next.run(req).await)
}
