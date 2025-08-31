use crate::helpers::response::create_response;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};

#[derive(Serialize, FromRow, Debug)]
pub struct User {
    id: u32,
    name: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateUserReq {
    name: String,
}

pub async fn user_get(
    State(pool): State<MySqlPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let users = sqlx::query_as!(User, "SELECT id, name FROM users")
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                create_response(false, "Failed to get all users", e.to_string()),
            )
        })?;

    Ok((
        StatusCode::OK,
        create_response(true, "get all users successfully", users),
    ))
}

pub async fn user_get_by_id(
    State(pool): State<MySqlPool>,
    Path(id): Path<u32>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let user = sqlx::query_as!(User, "SELECT id, name FROM users WHERE id = ?", id)
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                create_response(false, "Failed to get user by id", e.to_string()),
            )
        })?;
    Ok((
        StatusCode::OK,
        create_response(true, "get user by id successfully", user),
    ))
}

pub async fn user_create(
    State(pool): State<MySqlPool>,
    Json(user): Json<CreateUserReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    sqlx::query_as!(
        CreateUserRes,
        "INSERT INTO users (name) VALUES (?)",
        user.name
    )
    .execute(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            create_response(false, "Failed to create user", e.to_string()),
        )
    })?;

    Ok((
        StatusCode::OK,
        create_response(true, "create user successfully", {}),
    ))
}

pub async fn user_update(
    State(pool): State<MySqlPool>,
    Path(id): Path<u32>,
    Json(user): Json<CreateUserReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    sqlx::query_as!(
        User,
        "UPDATE users SET name = ? WHERE id = ?",
        user.name,
        id
    )
    .execute(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            create_response(false, "Failed to update user", e.to_string()),
        )
    })?;

    Ok((
        StatusCode::OK,
        create_response(true, "update user successfully", {}),
    ))
}

pub async fn user_delete(
    State(pool): State<MySqlPool>,
    Path(id): Path<u32>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    sqlx::query_as!(User, "DELETE FROM users WHERE id = ?", id)
        .execute(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                create_response(false, "Failed to delete user", e.to_string()),
            )
        })?;

    Ok((
        StatusCode::OK,
        create_response(true, "delete user successfully", {}),
    ))
}
