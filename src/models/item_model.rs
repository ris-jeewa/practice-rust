use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemModel {
    #[serde(skip_deserializing, default)]
    pub id: Option<i32>,
    pub product_id: i32,
    pub color: String,
    pub stock: i32,
    pub size: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetItemModel{
    pub product_id: i32,
    pub color: String,
    pub size: String,
    pub stock: i32,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct UpdateItemModel{
    pub size: Option<String>,
    pub color: Option<String>,
    pub stock: Option<i32>,
}


