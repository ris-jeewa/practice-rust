use axum::{extract::{Path, State}, http::{Method, StatusCode}, response::IntoResponse, routing::post, Extension, Json, Router};
use chrono::{NaiveDateTime, Utc};
use futures::stream::Any;
use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, TransactionTrait};
use tower_http::cors::CorsLayer;
use tracing::{info, instrument,error};

use crate::{entities::{item, product}, models::{item_model::ItemModel, product_model::{CreateProductModal, ProductItemModel, UpdateProductModal, WholeProductModel}, ErrorModel}, services::product_service::{self, ProductService}};


pub async fn create_product(
    State(service):State<ProductService>,
    Json(product_data): Json<CreateProductModal>,
) -> impl IntoResponse{
    
    match service.create_product(product_data).await {
        Ok(product) => {
            info!("Product created successfully");
            Ok((StatusCode::CREATED,Json(product)))
        }
        Err(ErrorModel::ValidationError(msg)) => {
            error!("Failed to create product: {}", msg);
            Err((StatusCode::BAD_REQUEST,Json(serde_json::json!({"error":msg}))))
            
        }
        Err(ErrorModel::DatabaseError(msg)) => {
            error!("Failed to create product: {}", msg);
            Err((StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":msg}))))
        }
    }

    
    
}


#[instrument(skip(db))]
pub async fn get_all_products(
    Extension(db): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    info!("Fetching all products with related items");

    match product::Entity::find()
        .find_with_related(item::Entity)
        .all(&db)
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
                    .collect()
                })
                .collect();

            info!("Successfully fetched {} products", response.len());

            (StatusCode::OK, Json(response))
        }
        Err(err) => {
            error!("Failed to fetch products: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(vec![]),
            )
        }
    }
}

// pub async fn create_product(
//     State(service):State<ProductService>,
//     Json(product_data): Json<CreateProductModal>,
// ) -> Result<(StatusCode, Json<WholeProductModel>), (StatusCode, Json<serde_json::Value>)> {
    
//     match service.create_product(product_data).await {
//         Ok(product) => {
//             info!("Product created successfully");
//             Ok((StatusCode::CREATED,Json(product)))
//         }
//         Err(ErrorModel::ValidationError(msg)) => {
//             error!("Failed to create product: {}", msg);
//             Err((StatusCode::BAD_REQUEST,Json(serde_json::json!({"error":msg}))))
            
//         }
//         Err(ErrorModel::DatabaseError(msg)) => {
//             error!("Failed to create product: {}", msg);
//             Err((StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":msg}))))
//         }
//     }

    
    
// }




#[instrument(skip(db, product_data))]
pub async fn update_product(
    Extension(db): Extension<DatabaseConnection>,
    Path(product_id): Path<i32>,
    Json(product_data): Json<UpdateProductModal>,
) -> impl IntoResponse {
    info!("Updating product with ID: {}", product_id);

    let now: NaiveDateTime = Utc::now().naive_utc();

    // Fetch the product to update
    let product_result = product::Entity::find()
        .filter(product::Column::Id.eq(product_id))
        .one(&db)
        .await;

    match product_result {
        Ok(Some(existing_product)) => {
            let mut updated_product: product::ActiveModel = existing_product.into();

            updated_product.name = match product_data.name {
                Some(name) => Set(name),
                None => NotSet,
            };

            updated_product.description = Set(product_data.description);
            updated_product.updated_at = Set(now);

            match updated_product.update(&db).await {
                Ok(_) => {
                    info!("Product with ID {} updated successfully", product_id);
                    if let Err(e) = db.close().await {
                        error!("Failed to close the database connection: {:?}", e);
                    }
                    (StatusCode::ACCEPTED, "Product updated")
                }
                Err(err) => {
                    error!("Failed to update product with ID {}: {:?}", product_id, err);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to update product",
                    )
                }
            }
        }
        Ok(None) => {
            error!("Product with ID {} not found", product_id);
            (
                StatusCode::NOT_FOUND,
                "Product not found",
            )
        }
        Err(err) => {
            error!("Failed to fetch product with ID {}: {:?}", product_id, err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch product",
            )
        }
    }
}

#[instrument(skip(db))]
pub async fn delete_product(
    Extension(db): Extension<DatabaseConnection>,
    Path(product_id): Path<i32>,
) -> impl IntoResponse {
    info!("Deleting product with ID: {}", product_id);

    match db.begin().await {
        Ok(txn) => {
            // Delete associated items
            match item::Entity::delete_many()
                .filter(item::Column::ProductId.eq(product_id))
                .exec(&txn)
                .await
            {
                Ok(_) => {
                    // Delete the product
                    match product::Entity::delete_by_id(product_id).exec(&txn).await {
                        Ok(delete_result) => {
                            if delete_result.rows_affected > 0 {
                                if txn.commit().await.is_ok() {
                                    info!("Product with ID {} and associated items successfully deleted", product_id);
                                    (StatusCode::OK, Json("Product and associated items deleted")).into_response()
                                } else {
                                    error!("Failed to commit transaction for product ID {}", product_id);
                                    (StatusCode::INTERNAL_SERVER_ERROR, Json("Failed to commit transaction")).into_response()
                                }
                            } else {
                                txn.rollback().await.ok();
                                info!("Product with ID {} not found", product_id);
                                (StatusCode::NOT_FOUND, Json("Product not found")).into_response()
                            }
                        }
                        Err(err) => {
                            txn.rollback().await.ok();
                            error!("Failed to delete product ID {}: {:?}", product_id, err);
                            (StatusCode::INTERNAL_SERVER_ERROR, Json("Failed to delete product")).into_response()
                        }
                    }
                }
                Err(err) => {
                    txn.rollback().await.ok();
                    error!("Failed to delete associated items for product ID {}: {:?}", product_id, err);
                    (StatusCode::INTERNAL_SERVER_ERROR, Json("Failed to delete associated items")).into_response()
                }
            }
        }
        Err(err) => {
            error!("Failed to start transaction for product ID {}: {:?}", product_id, err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json("Failed to start transaction")).into_response()
        }
    }
}