use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use crate::{AppState, AppError};
use crate::models::{Config, CreateConfig, UpdateConfig, Component, CreateComponent, UpdateComponent, Project, CreateProject, UpdateProject, Template, CreateTemplate, UpdateTemplate};

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

// Config handlers

pub async fn get_config_list(
    State(state): State<AppState>,
    Query(_query): Query<PaginationQuery>,
) -> Result<Json<Vec<Config>>, AppError> {
    let records = Config::find_all(&state.db).await?;
    Ok(Json(records))
}

pub async fn create_config(
    State(state): State<AppState>,
    Json(new_record): Json<CreateConfig>,
) -> Result<Json<Config>, AppError> {
    let record = Config::create(&state.db, new_record).await?;
    Ok(Json(record))
}

pub async fn get_config_by_id(
    State(state): State<AppState>,
    Path(id): Path<Option<i32>>,
) -> Result<Json<Config>, AppError> {
    let record = Config::find_by_id(&state.db, id)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(record))
}

pub async fn update_config(
    State(state): State<AppState>,
    Path(id): Path<Option<i32>>,
    Json(updates): Json<UpdateConfig>,
) -> Result<Json<Config>, AppError> {
    let record = Config::update(&state.db, id, updates).await?;
    Ok(Json(record))
}

pub async fn delete_config(
    State(state): State<AppState>,
    Path(id): Path<Option<i32>>,
) -> Result<StatusCode, AppError> {
    Config::delete(&state.db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// Component handlers

pub async fn get_component_list(
    State(state): State<AppState>,
    Query(_query): Query<PaginationQuery>,
) -> Result<Json<Vec<Component>>, AppError> {
    let records = Component::find_all(&state.db).await?;
    Ok(Json(records))
}

pub async fn create_component(
    State(state): State<AppState>,
    Json(new_record): Json<CreateComponent>,
) -> Result<Json<Component>, AppError> {
    let record = Component::create(&state.db, new_record).await?;
    Ok(Json(record))
}

pub async fn get_component_by_id(
    State(state): State<AppState>,
    Path(id): Path<Option<i32>>,
) -> Result<Json<Component>, AppError> {
    let record = Component::find_by_id(&state.db, id)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(record))
}

pub async fn update_component(
    State(state): State<AppState>,
    Path(id): Path<Option<i32>>,
    Json(updates): Json<UpdateComponent>,
) -> Result<Json<Component>, AppError> {
    let record = Component::update(&state.db, id, updates).await?;
    Ok(Json(record))
}

pub async fn delete_component(
    State(state): State<AppState>,
    Path(id): Path<Option<i32>>,
) -> Result<StatusCode, AppError> {
    Component::delete(&state.db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// Project handlers

pub async fn get_project_list(
    State(state): State<AppState>,
    Query(_query): Query<PaginationQuery>,
) -> Result<Json<Vec<Project>>, AppError> {
    let records = Project::find_all(&state.db).await?;
    Ok(Json(records))
}

pub async fn create_project(
    State(state): State<AppState>,
    Json(new_record): Json<CreateProject>,
) -> Result<Json<Project>, AppError> {
    let record = Project::create(&state.db, new_record).await?;
    Ok(Json(record))
}

pub async fn get_project_by_id(
    State(state): State<AppState>,
    Path(id): Path<Option<i32>>,
) -> Result<Json<Project>, AppError> {
    let record = Project::find_by_id(&state.db, id)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(record))
}

pub async fn update_project(
    State(state): State<AppState>,
    Path(id): Path<Option<i32>>,
    Json(updates): Json<UpdateProject>,
) -> Result<Json<Project>, AppError> {
    let record = Project::update(&state.db, id, updates).await?;
    Ok(Json(record))
}

pub async fn delete_project(
    State(state): State<AppState>,
    Path(id): Path<Option<i32>>,
) -> Result<StatusCode, AppError> {
    Project::delete(&state.db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// Template handlers

pub async fn get_template_list(
    State(state): State<AppState>,
    Query(_query): Query<PaginationQuery>,
) -> Result<Json<Vec<Template>>, AppError> {
    let records = Template::find_all(&state.db).await?;
    Ok(Json(records))
}

pub async fn create_template(
    State(state): State<AppState>,
    Json(new_record): Json<CreateTemplate>,
) -> Result<Json<Template>, AppError> {
    let record = Template::create(&state.db, new_record).await?;
    Ok(Json(record))
}

pub async fn get_template_by_id(
    State(state): State<AppState>,
    Path(id): Path<Option<i32>>,
) -> Result<Json<Template>, AppError> {
    let record = Template::find_by_id(&state.db, id)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(record))
}

pub async fn update_template(
    State(state): State<AppState>,
    Path(id): Path<Option<i32>>,
    Json(updates): Json<UpdateTemplate>,
) -> Result<Json<Template>, AppError> {
    let record = Template::update(&state.db, id, updates).await?;
    Ok(Json(record))
}

pub async fn delete_template(
    State(state): State<AppState>,
    Path(id): Path<Option<i32>>,
) -> Result<StatusCode, AppError> {
    Template::delete(&state.db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

