use axum::{
    http::{request, StatusCode},
    response::IntoResponse,
};
use chrono::{NaiveDateTime, Utc};
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, DatabaseConnection, EntityTrait,
    ModelTrait, QueryFilter, Set,
};

use crate::{
    entities::{item, product},
    models::{
        item_model::ItemModel,
        product_model::{
            CreateProductModal, ProductItemModel, UpdateProductModal, WholeProductModel,
        },
        ErrorModel, NotFoundErrorModel,
    },
};

#[derive(Clone)]
pub struct ProductRepository {
    db: DatabaseConnection,
}

impl ProductRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create_product_in_db(
        &self,
        request: CreateProductModal,
    ) -> Result<WholeProductModel, ErrorModel> {
        let now: NaiveDateTime = Utc::now().naive_utc();

        let product_model = product::ActiveModel {
            name: Set(request.name.to_owned()),
            description: Set(request.description.to_owned()),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };

        match product_model.insert(&self.db).await {
            Ok(inserted_product) => Ok(WholeProductModel {
                id: inserted_product.id,
                name: inserted_product.name,
                description: inserted_product.description,
                created_at: inserted_product.created_at,
                updated_at: inserted_product.updated_at,
            }),
            Err(_) => Err(ErrorModel::DatabaseError(
                "Failed to create product".to_string(),
            )),
        }
    }

    pub async fn get_all_products_from_db(&self) -> Result<Vec<ProductItemModel>, ErrorModel> {
        match product::Entity::find()
            .find_with_related(item::Entity)
            .all(&self.db)
            .await
        {
            Ok(products_with_items) => {
                let response: Vec<ProductItemModel> = products_with_items
                    .into_iter()
                    .map(|(product, items)| ProductItemModel {
                        id: product.id,
                        name: product.name,
                        description: product.description,
                        items: items
                            .into_iter()
                            .map(|item| ItemModel {
                                id: Some(item.id),
                                product_id: item.product_id,
                                color: item.color.unwrap_or_default(),
                                size: item.size.unwrap_or_default(),
                                stock: item.stock,
                            })
                            .collect(),
                    })
                    .collect();

                Ok(response)
            }
            Err(_) => Err(ErrorModel::DatabaseError(
                "Failed to fetch all products".to_string(),
            )),
        }
    }

    pub async fn update_product_in_db(
        &self,
        product_id: i32,
        product_data: UpdateProductModal,
    ) -> Result<WholeProductModel, NotFoundErrorModel> {
        let now: NaiveDateTime = Utc::now().naive_utc();

        let product_result = product::Entity::find()
            .filter(product::Column::Id.eq(product_id))
            .one(&self.db)
            .await;

        match product_result {
            Ok(Some(existing_product)) => {
                // Convert the fetched model into an ActiveModel for update
                let mut updated_product: product::ActiveModel = existing_product.clone().into();

                // Update fields based on input data
                if let Some(name) = product_data.name {
                    updated_product.name = Set(name);
                }

                updated_product.description = Set(product_data.description);

                updated_product.updated_at = Set(now);

                match updated_product.update(&self.db).await {
                    Ok(_) => match self.find_product(product_id).await {
                        Ok(Some(updated_product)) => Ok(WholeProductModel {
                            id: updated_product.id,
                            name: updated_product.name,
                            description: updated_product.description,
                            created_at: updated_product.created_at,
                            updated_at: updated_product.updated_at,
                        }),
                        Ok(None) => Err(NotFoundErrorModel::NotFoundError(
                            "Product not found after update".to_string(),
                        )),
                        Err(err) => Err(NotFoundErrorModel::DatabaseError(format!(
                            "Failed to fetch updated product: {}",
                            err
                        ))),
                    },
                    Err(err) => Err(NotFoundErrorModel::DatabaseError(format!(
                        "Failed to update product: {}",
                        err
                    ))),
                }
            }
            Ok(None) => Err(NotFoundErrorModel::NotFoundError(
                "Product not found".to_string(),
            )),
            Err(err) => Err(NotFoundErrorModel::DatabaseError(format!(
                "Failed to fetch product: {}",
                err
            ))),
        }
    }

    pub async fn find_product(
        &self,
        product_id: i32,
    ) -> Result<Option<product::Model>, sea_orm::DbErr> {
        product::Entity::find_by_id(product_id).one(&self.db).await
    }

   
    pub async fn delete_product_in_db(&self, product_id: i32) -> Result<bool, NotFoundErrorModel> {
        let product_result = self.find_product(product_id)
            .await;

        match product_result {
            Ok(Some(existing_product)) => match existing_product.delete(&self.db).await {
                Ok(_) => Ok(true),
                Err(err) => Err(NotFoundErrorModel::DatabaseError(format!(
                    "Failed to delete product: {}",
                    err
                ))),
            },
            Ok(None) => Err(NotFoundErrorModel::NotFoundError(
                "Product not found".to_string(),
            )),
            Err(err) => Err(NotFoundErrorModel::DatabaseError(format!(
                "Failed to fetch product: {}",
                err
            ))),
        }
    }

    // pub async fn delete_associated_items(
    //     &self,
    //     product_id: i32,
    // ) -> Result<bool, NotFoundErrorModel> {
    //     match item::Entity::delete_many()
    //         .filter(item::Column::ProductId.eq(product_id))
    //         .exec(&txn)
    //         .await
    //     {
    //         Ok(_) => Ok(true),
    //         Err(err) => Err(NotFoundErrorModel::DatabaseError(format!(
    //             "Failed to delete associated items: {}",
    //             err
    //         ))),
    //     }
    // }
}
