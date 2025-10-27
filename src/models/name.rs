use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Name {
    pub id: i32,
    pub name: String,
    pub count: Option<i64>,
    pub category: Option<String>,
}

