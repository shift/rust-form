use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use crate::{AppState, AppError, Todo, CreateTodo, UpdateTodo};

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

// Todo handlers

pub async fn get_todo_list(
    State(state): State<AppState>,
    Query(_query): Query<PaginationQuery>,
) -> Result<Json<Vec<Todo>>, AppError> {
    let records = Todo::find_all(&state.db).await?;
    Ok(Json(records))
}

pub async fn create_todo(
    State(state): State<AppState>,
    Json(new_record): Json<CreateTodo>,
) -> Result<Json<Todo>, AppError> {
    let record = Todo::create(&state.db, new_record).await?;
    Ok(Json(record))
}

pub async fn get_todo_by_id(
    State(state): State<AppState>,
    Path(id): Path<Option<i32>>,
) -> Result<Json<Todo>, AppError> {
    let record = Todo::find_by_id(&state.db, id)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(record))
}

pub async fn update_todo(
    State(state): State<AppState>,
    Path(id): Path<Option<i32>>,
    Json(updates): Json<UpdateTodo>,
) -> Result<Json<Todo>, AppError> {
    let record = Todo::update(&state.db, id, updates).await?;
    Ok(Json(record))
}

pub async fn delete_todo(
    State(state): State<AppState>,
    Path(id): Path<Option<i32>>,
) -> Result<StatusCode, AppError> {
    Todo::delete(&state.db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

