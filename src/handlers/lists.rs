use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    error::{AppError, Result},
    models::{CreateListRequest, List, ListWithCount, UpdateListRequest},
    state::AppState,
    validation,
};

/// GET /api/lists - Get all lists with item counts
pub async fn get_all_lists(State(state): State<AppState>) -> Result<Json<Vec<ListWithCount>>> {
    let lists = sqlx::query_as::<_, ListWithCount>(
        r#"
        SELECT l.id, l.name, (SELECT COUNT(*) FROM items WHERE "list" = l.id) as count
        FROM lists l
        ORDER BY id ASC
        "#,
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(lists))
}

/// GET /api/lists/:id - Get a single list
pub async fn get_list(State(state): State<AppState>, Path(id): Path<i32>) -> Result<Json<List>> {
    let list = sqlx::query_as::<_, List>(
        r#"
        SELECT id, name
        FROM lists
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(Json(list))
}

/// POST /api/lists - Create a new list
pub async fn create_list(
    State(state): State<AppState>,
    Json(payload): Json<CreateListRequest>,
) -> Result<(StatusCode, Json<List>)> {
    // Validate input
    validation::validate_string(&payload.name, "List name")?;

    let list = sqlx::query_as::<_, List>(
        r#"
        INSERT INTO lists (name)
        VALUES ($1)
        RETURNING id, name
        "#,
    )
    .bind(&payload.name)
    .fetch_one(&state.pool)
    .await?;

    Ok((StatusCode::CREATED, Json(list)))
}

/// PUT /api/lists/:id - Update a list (rename)
pub async fn update_list(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateListRequest>,
) -> Result<Json<List>> {
    // Validate input
    validation::validate_string(&payload.name, "List name")?;

    let list = sqlx::query_as::<_, List>(
        r#"
        UPDATE lists
        SET name = $1
        WHERE id = $2
        RETURNING id, name
        "#,
    )
    .bind(&payload.name)
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(Json(list))
}

/// DELETE /api/lists/:id - Delete a list
pub async fn delete_list(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode> {
    let result = sqlx::query(
        r#"
        DELETE FROM lists
        WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(&state.pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}

