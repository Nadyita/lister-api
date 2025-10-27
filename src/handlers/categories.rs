use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    error::{AppError, Result},
    models::Category,
    state::AppState,
};

#[derive(serde::Deserialize)]
pub struct CreateCategoryRequest {
    pub name: String,
}

#[derive(serde::Deserialize)]
pub struct UpdateCategoryRequest {
    pub name: String,
}

/// GET /api/categories - Get all categories
pub async fn get_all_categories(State(state): State<AppState>) -> Result<Json<Vec<Category>>> {
    let categories = sqlx::query_as::<_, Category>(
        r#"
        SELECT id, name
        FROM categories
        ORDER BY name ASC
        "#,
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(categories))
}

/// GET /api/categories/:id - Get a single category
pub async fn get_category(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Category>> {
    let category = sqlx::query_as::<_, Category>(
        r#"
        SELECT id, name
        FROM categories
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(Json(category))
}

/// POST /api/categories - Create a new category
pub async fn create_category(
    State(state): State<AppState>,
    Json(payload): Json<CreateCategoryRequest>,
) -> Result<(StatusCode, Json<Category>)> {
    let category = sqlx::query_as::<_, Category>(
        r#"
        INSERT INTO categories (name)
        VALUES ($1)
        RETURNING id, name
        "#,
    )
    .bind(&payload.name)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| {
        if let sqlx::Error::Database(ref db_err) = e {
            if db_err.is_unique_violation() {
                return AppError::BadRequest("Category already exists".to_string());
            }
        }
        AppError::Database(e)
    })?;

    Ok((StatusCode::CREATED, Json(category)))
}

/// PUT /api/categories/:id - Update a category
pub async fn update_category(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateCategoryRequest>,
) -> Result<Json<Category>> {
    let category = sqlx::query_as::<_, Category>(
        r#"
        UPDATE categories
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

    Ok(Json(category))
}

/// DELETE /api/categories/:id - Delete a category
pub async fn delete_category(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode> {
    let result = sqlx::query(
        r#"
        DELETE FROM categories
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

