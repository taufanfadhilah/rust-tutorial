use crate::dtos::user_dto::CreateUserReq;
use crate::models::user::User;
use axum::http::StatusCode;
use sqlx::MySqlPool;

pub async fn get_users(
    pool: &MySqlPool,
    limit: i64,
    offset: i64,
) -> Result<Vec<User>, (StatusCode, String)> {
    sqlx::query_as!(
        User,
        "SELECT id, name FROM users LIMIT ? OFFSET ?",
        limit,
        offset
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get all users: {}", e),
        )
    })
}

pub async fn count_users(pool: &MySqlPool) -> Result<i64, (StatusCode, String)> {
    let total: (i64,) = sqlx::query_as("SELECT COUNT(id) FROM users")
        .fetch_one(pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to count users: {}", e),
            )
        })?;
    Ok(total.0)
}

pub async fn get_user_by_id(pool: &MySqlPool, id: u32) -> Result<User, (StatusCode, String)> {
    sqlx::query_as!(User, "SELECT id, name FROM users WHERE id = ?", id)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to get user by id: {}", e),
            )
        })
}

pub async fn create_user(
    pool: &MySqlPool,
    user: CreateUserReq,
) -> Result<(), (StatusCode, String)> {
    sqlx::query!("INSERT INTO users (name) VALUES (?)", user.name)
        .execute(pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to create user: {}", e),
            )
        })?;
    Ok(())
}

pub async fn update_user(
    pool: &MySqlPool,
    id: u32,
    user: CreateUserReq,
) -> Result<(), (StatusCode, String)> {
    sqlx::query!("UPDATE users SET name = ? WHERE id = ?", user.name, id)
        .execute(pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to update user: {}", e),
            )
        })?;
    Ok(())
}

pub async fn delete_user(pool: &MySqlPool, id: u32) -> Result<(), (StatusCode, String)> {
    sqlx::query!("DELETE FROM users WHERE id = ?", id)
        .execute(pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to delete user: {}", e),
            )
        })?;
    Ok(())
}
