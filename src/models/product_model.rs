use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

use super::item_model::ItemModel;

#[derive(Clone, Serialize, Deserialize)]
pub struct WholeProductModel{
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ProductModel{
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CreateProductModal{
    pub name: String,
    pub description: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UpdateProductModal{
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ProductItemModel{
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub items: Vec<ItemModel>,
}