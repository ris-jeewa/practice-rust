use axum::{extract::{Path, State}, http::{Method, StatusCode}, response::IntoResponse, routing::post, Extension, Json, Router};
use chrono::{NaiveDateTime, Utc};
use futures::{future::ok, stream::Any};
use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, TransactionTrait};
use tower_http::cors::CorsLayer;
use tracing::{info, instrument,error};

use crate::{entities::{item, product}, models::{item_model::ItemModel, product_model::{CreateProductModal, ProductItemModel, UpdateProductModal, WholeProductModel}, ErrorModel, NotFoundErrorModel}, services::product_service::{self, ProductService}};


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


pub async fn get_all_products(
    State(service): State<ProductService>,
) -> impl IntoResponse {
    info!("Fetching all products with related items");
    match service.get_all_products().await {
        Ok(products) => {
            info!("Products fetched successfully");
            Ok((StatusCode::OK, Json(products)))
        }
        Err(ErrorModel::ValidationError(msg)) => {
            error!("Failed to create product: {}", msg);
            Err((StatusCode::BAD_REQUEST,Json(serde_json::json!({"error":msg}))))
            
        }
        Err(ErrorModel::DatabaseError(msg)) => {
            error!("Failed to fetch products: {}", msg);
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": msg})))
            )
        }
        
    }

}

pub async fn update_product(
    State(service): State<ProductService>,
    Path(product_id): Path<i32>,
    Json(product_data): Json<UpdateProductModal>,
) -> impl IntoResponse {
    match service.update_product(product_id, product_data).await {
        Ok(product) => {
            info!("Product with ID {} updated successfully", product_id);
            Ok((StatusCode::ACCEPTED, Json(product)))
        }
        Err(NotFoundErrorModel::ValidationError(msg)) => {
            error!("Product validation failed: {}", msg);
            Err((
                StatusCode::BAD_REQUEST, 
                Json(serde_json::json!({"error": msg}))
            ))
        }
        Err(NotFoundErrorModel::NotFoundError(msg)) => {
            error!("Failed to update product with ID {}: {}", product_id, msg);
            Err((
                StatusCode::NOT_FOUND, 
                Json(serde_json::json!({"error": msg}))
            ))
        }
        Err(NotFoundErrorModel::DatabaseError(msg)) => {
            error!("Database error when updating product: {}", msg);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR, 
                Json(serde_json::json!({"error": msg}))
            ))
        }
    }
}


pub async fn delete_product(
    State(service): State<ProductService>,
    Path(product_id): Path<i32>,
)-> impl IntoResponse{
    match service.delete_product(product_id).await {
        Ok(_) => {
            info!("Product with ID {} deleted successfully", product_id);
            Ok((StatusCode::OK, Json("Product deleted")))
        }
        Err(NotFoundErrorModel::ValidationError(msg)) => {
            error!("Product validation failed: {}", msg);
            Err((
                StatusCode::BAD_REQUEST, 
                Json(serde_json::json!({"error": msg}))
            ))
        }
        Err(NotFoundErrorModel::NotFoundError(msg)) => {
            error!("Failed to delete product with ID {}: {}", product_id, msg);
            Err((
                StatusCode::NOT_FOUND, 
                Json(serde_json::json!({"error": msg}))
            ))
        }
        Err(NotFoundErrorModel::DatabaseError(msg)) => {
            error!("Database error when deleting product: {}", msg);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR, 
                Json(serde_json::json!({"error": msg}))
            ))
        }
    }
    
}

// #[instrument(skip(db))]
// pub async fn delete_product(
//     Extension(db): Extension<DatabaseConnection>,
//     Path(product_id): Path<i32>,
// ) -> impl IntoResponse {
//     info!("Deleting product with ID: {}", product_id);

//     match db.begin().await {
//         Ok(txn) => {
//             // Delete associated items
//             match item::Entity::delete_many()
//                 .filter(item::Column::ProductId.eq(product_id))
//                 .exec(&txn)
//                 .await
//             {
//                 Ok(_) => {
//                     // Delete the product
//                     match product::Entity::delete_by_id(product_id).exec(&txn).await {
//                         Ok(delete_result) => {
//                             if delete_result.rows_affected > 0 {
//                                 if txn.commit().await.is_ok() {
//                                     info!("Product with ID {} and associated items successfully deleted", product_id);
//                                     (StatusCode::OK, Json("Product and associated items deleted")).into_response()
//                                 } else {
//                                     error!("Failed to commit transaction for product ID {}", product_id);
//                                     (StatusCode::INTERNAL_SERVER_ERROR, Json("Failed to commit transaction")).into_response()
//                                 }
//                             } else {
//                                 txn.rollback().await.ok();
//                                 info!("Product with ID {} not found", product_id);
//                                 (StatusCode::NOT_FOUND, Json("Product not found")).into_response()
//                             }
//                         }
//                         Err(err) => {
//                             txn.rollback().await.ok();
//                             error!("Failed to delete product ID {}: {:?}", product_id, err);
//                             (StatusCode::INTERNAL_SERVER_ERROR, Json("Failed to delete product")).into_response()
//                         }
//                     }
//                 }
//                 Err(err) => {
//                     txn.rollback().await.ok();
//                     error!("Failed to delete associated items for product ID {}: {:?}", product_id, err);
//                     (StatusCode::INTERNAL_SERVER_ERROR, Json("Failed to delete associated items")).into_response()
//                 }
//             }
//         }
//         Err(err) => {
//             error!("Failed to start transaction for product ID {}: {:?}", product_id, err);
//             (StatusCode::INTERNAL_SERVER_ERROR, Json("Failed to start transaction")).into_response()
//         }
//     }
// }