use axum::{extract::{Path, State}, http::{header::CONTENT_TYPE, StatusCode}, response::{IntoResponse, Json, Response}, Extension};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use tracing::{error, info, instrument, warn};

use crate::{entities::item, models::{item_model::{self, CreateItemModel, ItemModel, UpdateItemModel}, ErrorModel}, services::item_service::ItemService};


// #[instrument(skip(db, item_data))]
// pub async fn create_item(
//     Extension(db): Extension<DatabaseConnection>,
//     Json(item_data): Json<ItemModel>,
// ) -> impl IntoResponse {
//     info!("Creating a item");

//     let item = item::ActiveModel {
//         product_id: Set(item_data.product_id.to_owned()),
//         color: Set(Some(item_data.color.to_owned())), 
//         stock: Set(item_data.stock.to_owned()),
//         size: Set(Some(item_data.size.to_owned())),   
//         ..Default::default()
//     };

//     match item.insert(&db).await {
//         Ok(_result) => {
//             info!("Item created successfully with product_id: {}", item_data.product_id);
//             (StatusCode::CREATED, "Item created")
//         },
//         Err(err) => {
//             error!("Failed to create item: {:?}", err);
//             (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create item")
//         }
//     }
// }

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

#[instrument(skip(db))]
pub async fn delete_item(
    Extension(db): Extension<DatabaseConnection>,
    Path(item_id): Path<i32>,
) -> impl IntoResponse {
    info!("Attempting to delete item with ID: {}", item_id);

    // Directly attempt to delete the item
    match item::Entity::delete_by_id(item_id).exec(&db).await {
        Ok(delete_result) => {
            if delete_result.rows_affected > 0 {
                info!("Item with ID: {} deleted successfully", item_id);
                (StatusCode::OK, Json("Item deleted")).into_response()
            } else {
                info!("Item with ID: {} not found", item_id);
                (StatusCode::NOT_FOUND, Json("Item not found")).into_response()
            }
        },
        Err(err) => {
            error!("Failed to delete item with ID: {}: {:?}", item_id, err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json("Failed to delete item")).into_response()
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


#[instrument(skip(db))]
pub async fn get_item_by_id(
    Path(id): Path<i32>,                          // Extract the ID from the URL path
    Extension(db): Extension<DatabaseConnection>, // Pass the database connection
) -> Response {
    info!("Fetching item with ID: {}", id);

    match item::Entity::find_by_id(id).one(&db).await {
        Ok(Some(item)) => {
            info!("Successfully fetched item with ID: {}", id);
            
            let json_response = item_model::ItemModel {
                id: item.id,
                product_id: item.product_id,
                color: item.color,
                size: item.size,
                stock: item.stock,
            };
        
            Response::builder()
                .status(StatusCode::OK)
                .header(CONTENT_TYPE, "application/json")
                .body(serde_json::to_string(&json_response).unwrap().into())
                .unwrap()
        }

        Ok(None) => {
            warn!("Item with ID: {} not found", id);
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("Item not found".into())
                .unwrap()
        }
        Err(err) => {
            error!("Failed to retrieve item with ID: {}: {:?}", id, err);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body("Failed to retrieve item".into())
                .unwrap()
        }
    }
}