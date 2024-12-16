use sea_orm::DatabaseConnection;

use crate::{entities::item, models::{item_model::{CreateItemModel, ItemModel}, ErrorModel}};
use sea_orm::{
    ActiveModelTrait,Set,
};

#[derive(Clone)]
pub struct ItemRepository {
    db: DatabaseConnection,
}

impl ItemRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create_item_in_db(&self, request: CreateItemModel) -> Result<ItemModel, ErrorModel> {
        let item_model = item::ActiveModel {
            product_id:Set(request.product_id),
            color: Set(request.color),
            stock: Set(request.stock),
            size: Set(request.size),
            ..Default::default()
        };

        match item_model.insert(&self.db).await {
            Ok(inserted_item) => Ok(ItemModel {
                id:inserted_item.id,
                product_id: inserted_item.product_id,
                color: inserted_item.color,
                stock: inserted_item.stock,
                size: inserted_item.size,
            }),
            Err(_) => Err(ErrorModel::DatabaseError(
                "Failed to create item".to_string(),
            )),
        }
    }
}