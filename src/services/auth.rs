//! Authentication service — registration, login, token verification.

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;

use crate::{
    db::DbPool,
    errors::ApiError,
    models::user::{Claims, User, UserRow},
};

/// Register a new user
pub async fn register(pool: &DbPool, email: &str, password: &str) -> Result<User, ApiError> {
    // Basic validation
    if email.is_empty() || !email.contains('@') {
        return Err(ApiError::BadRequest("invalid email format".into()));
    }
    if password.len() < 8 {
        return Err(ApiError::BadRequest(
            "password must be at least 8 characters".into(),
        ));
    }

    // Hash password (Argon2id with secure defaults)
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| ApiError::Internal(anyhow::anyhow!("password hashing failed: {e}")))?
        .to_string();

    // Insert user
    let row = sqlx::query_as::<_, UserRow>(
        r#"
        INSERT INTO users (id, email, password_hash)
        VALUES ($1, LOWER($2), $3)
        RETURNING id, email, password_hash, created_at, updated_at
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(email)
    .bind(&password_hash)
    .fetch_one(pool)
    .await
    .map_err(|e| {
        if e.to_string().contains("duplicate key") || e.to_string().contains("unique") {
            ApiError::Conflict
        } else {
            ApiError::Database(e)
        }
    })?;

    Ok(row.into())
}

/// Login — verify credentials and issue JWT
pub async fn login(
    pool: &DbPool,
    email: &str,
    password: &str,
    jwt_secret: &str,
    jwt_expiration_hours: i64,
) -> Result<(String, User), ApiError> {
    // Fetch user with password_hash
    let row = sqlx::query_as::<_, UserRow>(
        "SELECT id, email, password_hash, created_at, updated_at FROM users WHERE LOWER(email) = LOWER($1)",
    )
    .bind(email)
    .fetch_optional(pool)
    .await?
    .ok_or(ApiError::Unauthorized)?;

    // Verify password (constant-time)
    let parsed_hash = PasswordHash::new(&row.password_hash)
        .map_err(|e| ApiError::Internal(anyhow::anyhow!("invalid stored hash: {e}")))?;
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| ApiError::Unauthorized)?;

    // Generate JWT
    let exp = Utc::now().timestamp() + (jwt_expiration_hours * 3600);
    let claims = Claims {
        sub: row.id.to_string(),
        email: row.email.clone(),
        exp,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|e| ApiError::Internal(anyhow::anyhow!("JWT encoding failed: {e}")))?;

    Ok((token, row.into()))
}

/// Verify JWT and extract claims
pub fn verify_token(token: &str, jwt_secret: &str) -> Result<Claims, ApiError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| ApiError::Unauthorized)?;

    Ok(token_data.claims)
}
