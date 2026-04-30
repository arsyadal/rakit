//! RBAC helpers.

use crate::{db::DbPool, errors::ApiError, models::rbac::{Permission, Role}};
use uuid::Uuid;

pub async fn list_roles(pool: &DbPool) -> Result<Vec<Role>, ApiError> {
    let roles = sqlx::query_as::<_, Role>(
        "SELECT id, name, created_at, updated_at FROM roles ORDER BY name ASC",
    )
    .fetch_all(pool)
    .await?;
    Ok(roles)
}

pub async fn list_permissions(pool: &DbPool) -> Result<Vec<Permission>, ApiError> {
    let permissions = sqlx::query_as::<_, Permission>(
        "SELECT id, action, collection, created_at, updated_at FROM permissions ORDER BY action, collection",
    )
    .fetch_all(pool)
    .await?;
    Ok(permissions)
}

pub async fn get_role_id(pool: &DbPool, role_name: &str) -> Result<Uuid, ApiError> {
    let role_id = sqlx::query_scalar::<_, Uuid>("SELECT id FROM roles WHERE name = $1")
        .bind(role_name)
        .fetch_optional(pool)
        .await?
        .ok_or(ApiError::NotFound)?;
    Ok(role_id)
}

pub async fn get_user_role_name(pool: &DbPool, user_id: Uuid) -> Result<String, ApiError> {
    let role_name = sqlx::query_scalar::<_, String>(
        r#"
        SELECT r.name
        FROM user_roles ur
        JOIN roles r ON r.id = ur.role_id
        WHERE ur.user_id = $1
        ORDER BY CASE r.name
            WHEN 'admin' THEN 0
            WHEN 'editor' THEN 1
            WHEN 'viewer' THEN 2
            ELSE 3
        END
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?
    .ok_or(ApiError::Unauthorized)?;
    Ok(role_name)
}

pub async fn assign_role_to_user(pool: &DbPool, user_id: Uuid, role_name: &str) -> Result<(), ApiError> {
    let role_id = get_role_id(pool, role_name).await?;
    sqlx::query("DELETE FROM user_roles WHERE user_id = $1")
        .bind(user_id)
        .execute(pool)
        .await?;
    sqlx::query("INSERT INTO user_roles (user_id, role_id) VALUES ($1, $2)")
        .bind(user_id)
        .bind(role_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn has_permission(
    pool: &DbPool,
    role_name: &str,
    action: &str,
    collection: &str,
) -> Result<bool, ApiError> {
    let allowed = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS (
            SELECT 1
            FROM roles r
            JOIN role_permissions rp ON rp.role_id = r.id
            JOIN permissions p ON p.id = rp.permission_id
            WHERE r.name = $1
              AND p.action = $2
              AND (p.collection = $3 OR p.collection = '*')
        )
        "#,
    )
    .bind(role_name)
    .bind(action)
    .bind(collection)
    .fetch_one(pool)
    .await?;
    Ok(allowed)
}
