use sea_orm::{ActiveValue::NotSet, DatabaseConnection, EntityTrait};

use crate::{entities::item, models::{item_model::{CreateItemModel, ItemModel, UpdateItemModel}, ErrorModel, NotFoundErrorModel}};
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

    pub async fn delete_item_in_db(&self, item_id: i32) -> Result<bool, NotFoundErrorModel> {
        match self.find_item(item_id).await {
            Ok(item) => {
                match item::Entity::delete_by_id(item_id).exec(&self.db).await {
                    Ok(delete_result) => {
                        if delete_result.rows_affected > 0 {
                            Ok(true)
                        } else {
                            Ok(false)
                        }
                    },
                    Err(err) => Err(NotFoundErrorModel::DatabaseError(format!(
                        "Failed to delete item: {}",
                        err
                    ))),
                }
            }
            Err(err) => Err(NotFoundErrorModel::DatabaseError(format!(
                "Failed to update product: {}",
                err
            )))
        }

        
    }

    pub async fn update_item_in_db(
        &self,
        item_id: i32,
        item_data: UpdateItemModel,
    ) -> Result<ItemModel, NotFoundErrorModel> {
        match self.find_item(item_id).await {
            Ok(Some(item)) => {
                let mut updated_item: item::ActiveModel = item.into();
    
                updated_item.size = match item_data.size {
                    Some(size) => Set(size),
                    None => NotSet,
                };
                updated_item.color = match item_data.color {
                    Some(color) => Set(color),
                    None => NotSet,
                };
                updated_item.stock = match item_data.stock {
                    Some(stock) => Set(stock),
                    None => NotSet,
                };
    
                match updated_item.update(&self.db).await {
                    Ok(updated_item) => Ok(ItemModel {
                        id: updated_item.id,
                        product_id: updated_item.product_id,
                        color: updated_item.color,
                        stock: updated_item.stock,
                        size: updated_item.size,
                    }),
                    Err(err) => Err(NotFoundErrorModel::DatabaseError(format!(
                        "Failed to update item: {}",
                        err
                    ))),
                }
            }
            Ok(None) => Err(NotFoundErrorModel::NotFoundError(format!(
                "Item with ID {} not found",
                item_id
            ))),
            Err(err) => Err(NotFoundErrorModel::DatabaseError(format!(
                "Failed to retrieve item: {}",
                err
            ))),
        }
    }
    

    pub async fn get_item_by_id_from_db(&self,item_id: i32) -> Result<ItemModel, NotFoundErrorModel> {
        match self.find_item(item_id).await {
            Ok(Some(item)) => Ok(ItemModel {
                id: item.id,
                product_id: item.product_id,
                color: item.color,
                size: item.size,
                stock: item.stock,
            }),
            Ok(None) => Err(NotFoundErrorModel::NotFoundError(
                "Item not found".to_string(),
            )),
            Err(err) => Err(NotFoundErrorModel::DatabaseError(format!(
                "Failed to fetch item: {}",
                err
            ))),
        }
        
    }

    pub async fn find_item(
        &self,
        item_id: i32,
    ) -> Result<Option<item::Model>, sea_orm::DbErr> {
        item::Entity::find_by_id(item_id).one(&self.db).await
    }
    
}