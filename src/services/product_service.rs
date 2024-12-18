use crate::{
    models::{
        product_model::{
            CreateProductModal, ProductItemModel, UpdateProductModal, WholeProductModel,
        },
        ErrorModel, NotFoundErrorModel,
    },
    repositories::product_repository::ProductRepository,
};

#[derive(Clone)]
pub struct ProductService {
    product_repository: ProductRepository,
}

impl ProductService {
    pub fn new(product_repository: ProductRepository) -> Self {
        Self { product_repository }
    }

    pub async fn create_product(
        &self,
        request: CreateProductModal,
    ) -> Result<WholeProductModel, ErrorModel> {
        if request.name.is_empty() {
            return Err(ErrorModel::ValidationError("Name is required".to_string()));
        }

        self.product_repository.create_product_in_db(request).await
    }

    pub async fn get_all_products(&self) -> Result<Vec<ProductItemModel>, ErrorModel> {
        self.product_repository.get_all_products_from_db().await
    }

    pub async fn update_product(
        &self,
        product_id: i32,
        request: UpdateProductModal,
    ) -> Result<WholeProductModel, NotFoundErrorModel> {
        self.product_repository
            .update_product_in_db(product_id, request)
            .await
    }

    pub async fn delete_product(&self, product_id: i32) -> Result<bool, NotFoundErrorModel> {
        self.product_repository
            .delete_product_in_db(product_id)
            .await
    }
}
