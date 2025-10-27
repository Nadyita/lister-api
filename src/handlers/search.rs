use axum::{extract::State, Json};
use serde::Serialize;
use std::collections::HashMap;

use crate::{error::Result, state::AppState};

#[derive(Serialize)]
pub struct SearchResponse {
    pub names: Vec<String>,
}

#[derive(Serialize)]
pub struct CategoryByProductResponse {
    pub mappings: HashMap<String, Option<String>>,
}

/// GET /api/search - Get all known item names for autocomplete
pub async fn search_names(State(state): State<AppState>) -> Result<Json<Vec<String>>> {
    let names = sqlx::query_scalar::<_, String>(
        r#"
        SELECT name
        FROM names
        ORDER BY count DESC, name ASC
        "#,
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(names))
}

/// GET /api/search/category-mappings - Get product name to category mappings
pub async fn get_category_mappings(
    State(state): State<AppState>,
) -> Result<Json<HashMap<String, Option<String>>>> {
    let rows = sqlx::query_as::<_, (String, Option<String>)>(
        r#"
        SELECT name, category
        FROM names
        "#,
    )
    .fetch_all(&state.pool)
    .await?;

    let mappings: HashMap<String, Option<String>> = rows.into_iter().collect();

    Ok(Json(mappings))
}

