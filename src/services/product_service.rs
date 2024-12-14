use axum::http::request;

use crate::{models::{product_model::{CreateProductModal, WholeProductModel}, ErrorModel}, repositories::product_repository::ProductRepository};

#[derive(Clone)]
pub struct ProductService {
    product_repository: ProductRepository,
}

impl ProductService {
    pub fn new(product_repository: ProductRepository) -> Self {
        Self {
            product_repository
        }
    }

    pub async fn create_product(&self, request: CreateProductModal) -> Result<WholeProductModel, ErrorModel> {
        if request.name.is_empty(){
            return Err(ErrorModel::ValidationError(
                "Name is required".to_string()
            ));
        }

        self.product_repository
            .create_product_in_db(request)
            .await
    }
}