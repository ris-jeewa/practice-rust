use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Clone, Serialize, Deserialize)]
pub struct ItemModel{
    pub id: i32,
    pub product_id: i32,
    pub size: Option<String>,
    pub color: Option<String>,
    pub stock: i32,
}

