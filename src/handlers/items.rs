use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    error::{AppError, Result},
    models::{CreateItemRequest, Item, UpdateItemRequest},
    state::AppState,
};

/// GET /api/lists/:list_id/items - Get all items in a list
pub async fn get_list_items(
    State(state): State<AppState>,
    Path(list_id): Path<i32>,
) -> Result<Json<Vec<Item>>> {
    let items = sqlx::query_as::<_, Item>(
        r#"
        SELECT id, name, amount, "amountUnit", "inCart", list, category
        FROM items
        WHERE list = $1
        ORDER BY id ASC
        "#,
    )
    .bind(list_id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(items))
}

/// GET /api/items/:id - Get a single item
pub async fn get_item(State(state): State<AppState>, Path(id): Path<i32>) -> Result<Json<Item>> {
    let item = sqlx::query_as::<_, Item>(
        r#"
        SELECT id, name, amount, "amountUnit", "inCart", list, category
        FROM items
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(Json(item))
}

/// POST /api/lists/:list_id/items - Create a new item
pub async fn create_item(
    State(state): State<AppState>,
    Path(list_id): Path<i32>,
    Json(payload): Json<CreateItemRequest>,
) -> Result<(StatusCode, Json<Item>)> {
    // Start transaction
    let mut tx = state.pool.begin().await?;

    // Insert category if provided and doesn't exist
    if let Some(ref category) = payload.category {
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

    // Insert or update name entry for autocomplete
    let existing_name = sqlx::query_scalar::<_, i32>(
        r#"
        SELECT id FROM names WHERE name = $1
        "#,
    )
    .bind(&payload.name)
    .fetch_optional(&mut *tx)
    .await?;

    if existing_name.is_some() {
        // Update count
        sqlx::query(
            r#"
            UPDATE names
            SET count = count + 1, category = COALESCE($2, category)
            WHERE name = $1
            "#,
        )
        .bind(&payload.name)
        .bind(&payload.category)
        .execute(&mut *tx)
        .await?;
    } else {
        // Insert new name
        sqlx::query(
            r#"
            INSERT INTO names (name, category, count)
            VALUES ($1, $2, 1)
            "#,
        )
        .bind(&payload.name)
        .bind(&payload.category)
        .execute(&mut *tx)
        .await?;
    }

    // Insert item
    let item = sqlx::query_as::<_, Item>(
        r#"
        INSERT INTO items (name, amount, "amountUnit", list, category, "inCart")
        VALUES ($1, $2, $3, $4, $5, false)
        RETURNING id, name, amount, "amountUnit", "inCart", list, category
        "#,
    )
    .bind(&payload.name)
    .bind(payload.amount)
    .bind(&payload.amount_unit)
    .bind(list_id)
    .bind(&payload.category)
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok((StatusCode::CREATED, Json(item)))
}

/// PUT /api/items/:id - Update an item
pub async fn update_item(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateItemRequest>,
) -> Result<Json<Item>> {
    // Start transaction
    let mut tx = state.pool.begin().await?;

    // Get current item
    let current_item = sqlx::query_as::<_, Item>(
        r#"
        SELECT id, name, amount, "amountUnit", "inCart", list, category
        FROM items
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or(AppError::NotFound)?;

    let new_name = payload.name.as_ref().unwrap_or(&current_item.name);
    let new_category = payload.category.or(current_item.category);

    // Insert category if provided and doesn't exist
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

    // Update names table category association
    sqlx::query(
        r#"
        UPDATE names
        SET category = $2
        WHERE name = $1
        "#,
    )
    .bind(new_name)
    .bind(&new_category)
    .execute(&mut *tx)
    .await?;

    // Update item
    let item = sqlx::query_as::<_, Item>(
        r#"
        UPDATE items
        SET name = COALESCE($1, name),
            amount = COALESCE($2, amount),
            "amountUnit" = COALESCE($3, "amountUnit"),
            category = $4
        WHERE id = $5
        RETURNING id, name, amount, "amountUnit", "inCart", list, category
        "#,
    )
    .bind(&payload.name)
    .bind(payload.amount.or(current_item.amount))
    .bind(&payload.amount_unit)
    .bind(new_category)
    .bind(id)
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(Json(item))
}

/// PATCH /api/items/:id/toggle - Toggle item in cart status
pub async fn toggle_item(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Item>> {
    let item = sqlx::query_as::<_, Item>(
        r#"
        UPDATE items
        SET "inCart" = NOT "inCart"
        WHERE id = $1
        RETURNING id, name, amount, "amountUnit", "inCart", list, category
        "#,
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(Json(item))
}

/// DELETE /api/items/:id - Delete an item
pub async fn delete_item(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode> {
    let result = sqlx::query(
        r#"
        DELETE FROM items
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

