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

// #[instrument(skip(db, item_data))]
// pub async fn update_item(
//     Extension(db): Extension<DatabaseConnection>,
//     Path(item_id): Path<i32>,
//     Json(item_data): Json<UpdateItemModel>,
// ) -> impl IntoResponse {
//     info!("Updating item with ID: {}", item_id);

//     // Fetch the product to update
//     let product_result = item::Entity::find_by_id(item_id)
//         .one(&db)
//         .await;

//     match product_result {
//         Ok(Some(existing_item)) => {
//             let mut updated_item: item::ActiveModel = existing_item.into();

//             updated_item.size = Set(item_data.size);
//             updated_item.color = Set(item_data.color);

//             updated_item.stock = Set(item_data.stock.unwrap_or(0)); 

//             match updated_item.update(&db).await {
//                 Ok(_) => {
//                     info!("Item with ID {} updated successfully", item_id);
//                     if let Err(e) = db.close().await {
//                         error!("Failed to close the database connection: {:?}", e);
//                     }
//                     (StatusCode::ACCEPTED, "Item updated")
//                 }
//                 Err(err) => {
//                     error!("Failed to update item with ID {}: {:?}", item_id, err);
//                     (
//                         StatusCode::INTERNAL_SERVER_ERROR,
//                         "Failed to update item",
//                     )
//                 }
//             }
//         }
//         Ok(None) => {
//             error!("Item with ID {} not found", item_id);
//             (
//                 StatusCode::NOT_FOUND,
//                 "Item not found",
//             )
//         }
//         Err(err) => {
//             error!("Failed to fetch item with ID {}: {:?}", item_id, err);
//             (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 "Failed to fetch item",
//             )
//         }
//     }
// }


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


// #[instrument(skip(db))]
// pub async fn get_item_by_id(
//     Path(id): Path<i32>,                          // Extract the ID from the URL path
//     Extension(db): Extension<DatabaseConnection>, // Pass the database connection
// ) -> Response {
//     info!("Fetching item with ID: {}", id);

//     match item::Entity::find_by_id(id).one(&db).await {
//         Ok(Some(item)) => {
//             info!("Successfully fetched item with ID: {}", id);
            
//             let json_response = item_model::ItemModel {
//                 id: item.id,
//                 product_id: item.product_id,
//                 color: item.color,
//                 size: item.size,
//                 stock: item.stock,
//             };
        
//             Response::builder()
//                 .status(StatusCode::OK)
//                 .header(CONTENT_TYPE, "application/json")
//                 .body(serde_json::to_string(&json_response).unwrap().into())
//                 .unwrap()
//         }

//         Ok(None) => {
//             warn!("Item with ID: {} not found", id);
//             Response::builder()
//                 .status(StatusCode::NOT_FOUND)
//                 .body("Item not found".into())
//                 .unwrap()
//         }
//         Err(err) => {
//             error!("Failed to retrieve item with ID: {}: {:?}", id, err);
//             Response::builder()
//                 .status(StatusCode::INTERNAL_SERVER_ERROR)
//                 .body("Failed to retrieve item".into())
//                 .unwrap()
//         }
//     }
// }

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