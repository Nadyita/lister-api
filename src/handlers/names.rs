use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    error::{AppError, Result},
    models::Name,
    state::AppState,
    validation,
};

#[derive(serde::Deserialize)]
pub struct UpdateNameRequest {
    pub name: Option<String>,
    pub category: Option<Option<String>>,
}

/// GET /api/names - Get all names
pub async fn get_all_names(State(state): State<AppState>) -> Result<Json<Vec<Name>>> {
    let names = sqlx::query_as::<_, Name>(
        r#"
        SELECT id, name, count, category
        FROM names
        ORDER BY count DESC, name ASC
        "#,
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(names))
}

/// GET /api/names/:id - Get a single name entry
pub async fn get_name(State(state): State<AppState>, Path(id): Path<i32>) -> Result<Json<Name>> {
    let name = sqlx::query_as::<_, Name>(
        r#"
        SELECT id, name, count, category
        FROM names
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(Json(name))
}

/// PUT /api/names/:id - Update a name entry
pub async fn update_name(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateNameRequest>,
) -> Result<Json<Name>> {
    // Validate input
    if let Some(ref name) = payload.name {
        validation::validate_string(name, "Name")?;
    }
    if let Some(Some(ref cat)) = payload.category {
        validation::validate_string(cat, "Category")?;
    }

    // Start transaction
    let mut tx = state.pool.begin().await?;

    // Get current name entry
    let current_name = sqlx::query_as::<_, Name>(
        r#"
        SELECT id, name, count, category
        FROM names
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or(AppError::NotFound)?;

    // Determine new values
    let new_name = payload.name.as_ref().unwrap_or(&current_name.name);
    let new_category = match payload.category {
        Some(inner) => inner,
        None => current_name.category.clone(),
    };

    // If category is provided and not null, ensure it exists in categories table
    if let Some(ref category) = new_category {
        sqlx::query(
            r#"
            INSERT INTO categories (name)
            VALUES ($1)
            ON CONFLICT (name) DO NOTHING
            "#,
        )
        .bind(category)
        .execute(&mut *tx)
        .await?;
    }

    // If the name itself changed, update all items that use this name
    if new_name != &current_name.name {
        sqlx::query(
            r#"
            UPDATE items
            SET name = $1
            WHERE name = $2
            "#,
        )
        .bind(new_name)
        .bind(&current_name.name)
        .execute(&mut *tx)
        .await?;
    }

    // If the category changed, update all items that use this name
    if new_category != current_name.category {
        sqlx::query(
            r#"
            UPDATE items
            SET category = $1
            WHERE name = $2
            "#,
        )
        .bind(&new_category)
        .bind(&current_name.name)
        .execute(&mut *tx)
        .await?;
    }

    // Update the name entry
    let updated_name = sqlx::query_as::<_, Name>(
        r#"
        UPDATE names
        SET name = $1, category = $2
        WHERE id = $3
        RETURNING id, name, count, category
        "#,
    )
    .bind(new_name)
    .bind(&new_category)
    .bind(id)
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(Json(updated_name))
}

/// DELETE /api/names/:id - Delete a name entry
pub async fn delete_name(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode> {
    let result = sqlx::query(
        r#"
        DELETE FROM names
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


