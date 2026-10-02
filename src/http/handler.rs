use crate::{
    error::Result,
    models::{NewTag, Tag},
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::SqlitePool;

pub async fn list_tags(State(pool): State<SqlitePool>) -> Result<Json<Vec<Tag>>> {
    let payload = Tag::find_all(&pool).await?;
    Ok(Json(payload))
}

pub async fn get_tag(
    Path(id): Path<i32>,
    State(pool): State<SqlitePool>,
) -> Result<impl IntoResponse> {
    let tag_option = Tag::find_by_id(id, &pool).await?;

    match tag_option {
        Some(tag) => Ok(Json(tag).into_response()),
        None => Ok(StatusCode::NOT_FOUND.into_response()),
    }
}

pub async fn create_tag(
    State(pool): State<SqlitePool>,
    Json(new_tag): Json<NewTag>,
) -> Result<impl IntoResponse> {
    let tag_exists = Tag::find_by_label(&new_tag.label, &pool).await?.is_some();

    if tag_exists {
        return Ok(StatusCode::CONFLICT.into_response());
    }

    let tag: Tag = new_tag.insert(&pool).await?;
    Ok((StatusCode::CREATED, Json(tag)).into_response())
}

pub async fn delete_tag(Path(id): Path<i32>, State(pool): State<SqlitePool>) -> Result<StatusCode> {
    let deleted = Tag::delete_by_id(id, &pool).await?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Ok(StatusCode::NOT_FOUND)
    }
}
