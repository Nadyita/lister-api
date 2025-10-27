use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct List {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ListWithCount {
    pub id: i32,
    pub name: String,
    pub count: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateListRequest {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateListRequest {
    pub name: String,
}

