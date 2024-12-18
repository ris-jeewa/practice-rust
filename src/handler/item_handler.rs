use axum::{extract::{Path, State}, http:: StatusCode, response::{IntoResponse, Json}};
use tracing::{error, info};

use crate::{models::{item_model::{ CreateItemModel,UpdateItemModel}, ErrorModel, NotFoundErrorModel}, services::item_service::ItemService};

pub async fn create_item(
    State(service): State<ItemService>,
    Json(item_data): Json<CreateItemModel>,
)-> impl IntoResponse{
    match service.create_item(item_data).await {
        Ok(item) => {
            info!("Item created successfully");
            Ok((StatusCode::CREATED,Json(item)))
        }
        Err(ErrorModel::ValidationError(msg)) => {
            error!("Failed to create item");
            Err((StatusCode::BAD_REQUEST,Json(serde_json::json!({"error":msg}))))
            
        }
        Err(ErrorModel::DatabaseError(msg)) => {
            error!("Failed to create item");
            Err((StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":msg}))))
        }
    }

}


pub async fn delete_item(
    State(service): State<ItemService>,
    Path(item_id): Path<i32>,
)-> impl IntoResponse {
    match service.delete_item(item_id).await {
        Ok(_) => {
            info!("Item deleted successfully");
            Ok((StatusCode::OK,Json("Item deleted")))
        }
        Err(NotFoundErrorModel::ValidationError(msg)) => {
            error!("Failed to delete item");
            Err((StatusCode::BAD_REQUEST,Json(serde_json::json!({"error":msg})))
            )
        }
        Err(NotFoundErrorModel::DatabaseError(msg)) => {
            error!("Failed to delete item");
            Err((StatusCode::INTERNAL_SERVER_ERROR,Json(serde_json::json!({"error":msg})))
            )
        }
        Err(NotFoundErrorModel::NotFoundError(msg)) => {
            error!("Failed to delete item");
            Err((StatusCode::NOT_FOUND,Json(serde_json::json!({"error":msg})))
            )
        }
    }
}

pub async fn update_item(
    State(service): State<ItemService>,
    Path(item_id): Path<i32>,
    Json(item_data): Json<UpdateItemModel>,
)->impl IntoResponse{
    match service.update_item(item_id,item_data).await {
        Ok(item) => {
            info!("Item with ID {} updated successfully", item_id);
            Ok((StatusCode::ACCEPTED,Json(item)))
        }
        Err(NotFoundErrorModel::ValidationError(msg)) => {
            error!("Item validation failed: {}", msg);
            Err((
                StatusCode::BAD_REQUEST, 
                Json(serde_json::json!({"error": msg}))
            ))
        }
        Err(NotFoundErrorModel::NotFoundError(msg)) => {
            error!("Failed to update item with ID {}: {}", item_id, msg);
            Err((
                StatusCode::NOT_FOUND, 
                Json(serde_json::json!({"error": msg}))
            ))
        }
        Err(NotFoundErrorModel::DatabaseError(msg)) => {
            error!("Database error when updating item: {}", msg);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR, 
                Json(serde_json::json!({"error": msg}))
            ))
        }
    }
}

pub async fn get_item_by_id(
    State(service): State<ItemService>,
    Path(item_id): Path<i32>,                       
) -> impl IntoResponse {
    match service.get_item_by_id(item_id).await {
        Ok(item) => {
            info!("Item fetched successfully");
            Ok((StatusCode::OK,Json(item)))
        }
        Err(NotFoundErrorModel::ValidationError(msg)) => {
            error!("Item validation failed: {}", msg);
            Err((
                StatusCode::BAD_REQUEST, 
                Json(serde_json::json!({"error": msg}))
            ))
        }
        Err(NotFoundErrorModel::NotFoundError(msg)) => {
            error!("Failed to fetch item");
            Err((
                StatusCode::NOT_FOUND, 
                Json(serde_json::json!({"error": msg}))
            ))
        }
        Err(NotFoundErrorModel::DatabaseError(msg)) => {
            error!("Database error when fetching item");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR, 
                Json(serde_json::json!({"error": msg}))
            ))
        }
        
    }
}