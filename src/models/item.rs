use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Item {
    pub id: i32,
    pub name: String,
    #[sqlx(rename = "amount")]
    pub amount: Option<Decimal>,
    #[sqlx(rename = "amountUnit")]
    #[serde(rename = "amountUnit")]
    pub amount_unit: Option<String>,
    #[sqlx(rename = "inCart")]
    #[serde(rename = "inCart")]
    pub in_cart: bool,
    pub list: i32,
    pub category: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateItemRequest {
    pub name: String,
    pub amount: Option<Decimal>,
    #[serde(rename = "amountUnit")]
    pub amount_unit: Option<String>,
    pub category: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateItemRequest {
    pub name: Option<String>,
    #[serde(default, deserialize_with = "deserialize_some")]
    pub amount: Option<Option<Decimal>>,
    #[serde(rename = "amountUnit")]
    #[serde(default, deserialize_with = "deserialize_some")]
    pub amount_unit: Option<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_some")]
    pub category: Option<Option<String>>,
}

// Helper function to distinguish between missing field and explicit null
fn deserialize_some<'de, T, D>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    T: Deserialize<'de>,
    D: serde::Deserializer<'de>,
{
    Deserialize::deserialize(deserializer).map(Some)
}

