use crate::{models::{item_model::{CreateItemModel, ItemModel}, ErrorModel}, repositories::item_repository::ItemRepository};

#[derive(Clone)]
pub struct ItemService {
    item_repository: ItemRepository,
}

impl ItemService {
    pub fn new(item_repository: ItemRepository) -> Self {
        Self { item_repository }
    }

    pub async fn create_item(&self, request: CreateItemModel) -> Result<ItemModel, ErrorModel> {
        if request.product_id == 0 {
            return Err(ErrorModel::ValidationError("Product ID is required".to_string()));
        }else if request.color.is_empty() {
            return Err(ErrorModel::ValidationError("Color is required".to_string()));
        }else if request.size.is_empty() {
            return Err(ErrorModel::ValidationError("Size is required".to_string()));
        }else if request.stock == 0 {
            return Err(ErrorModel::ValidationError("Stock is required".to_string()));
        }

        self.item_repository.create_item_in_db(request).await
    }
}