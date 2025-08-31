use crate::dtos::user_dto::CreateUserReq;
use crate::helpers::response::{create_pagination_metadata, create_response, Pagination};
use crate::services::user_service;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use sqlx::MySqlPool;

pub async fn user_get(
    State(pool): State<MySqlPool>,
    query: Query<Pagination>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let offset = (query.page.unwrap_or(1) - 1) * query.per_page.unwrap_or(10);

    let users = user_service::get_users(
        &pool,
        query.per_page.unwrap_or(10) as i64,
        offset as i64,
    ).await?;

    let total = user_service::count_users(&pool).await?;

    let metadata = create_pagination_metadata(users, total as usize, query);

    Ok((
        StatusCode::OK,
        create_response(true, "get all users successfully", metadata),
    ))
}

pub async fn user_get_by_id(
    State(pool): State<MySqlPool>,
    Path(id): Path<u32>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let user = user_service::get_user_by_id(&pool, id).await?;
    Ok((
        StatusCode::OK,
        create_response(true, "get user by id successfully", user),
    ))
}

pub async fn user_create(
    State(pool): State<MySqlPool>,
    Json(user): Json<CreateUserReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    user_service::create_user(&pool, user).await?;

    Ok((
        StatusCode::CREATED,
        create_response(true, "create user successfully", {}),
    ))
}

pub async fn user_update(
    State(pool): State<MySqlPool>,
    Path(id): Path<u32>,
    Json(user): Json<CreateUserReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    user_service::update_user(&pool, id, user).await?;

    Ok((
        StatusCode::OK,
        create_response(true, "update user successfully", {}),
    ))
}

pub async fn user_delete(
    State(pool): State<MySqlPool>,
    Path(id): Path<u32>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    user_service::delete_user(&pool, id).await?;

    Ok((
        StatusCode::OK,
        create_response(true, "delete user successfully", {}),
    ))
}
