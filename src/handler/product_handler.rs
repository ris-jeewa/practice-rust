use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse,Json};
use tracing::{info,error};

use crate::{models::{product_model::{CreateProductModal, UpdateProductModal}, ErrorModel, NotFoundErrorModel}, services::product_service::ProductService};


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
