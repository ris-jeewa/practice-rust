use axum::extract::Path;
use axum::routing::{delete, post, put};
use axum::Json;
use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use chrono::{NaiveDateTime, Utc};
use entities::product;
use entities::item;
use models::item_model::ItemModel;
use models::product_model::{self, CreateProductModal, ProductItemModel, ProductModel, UpdateProductModal};
use sea_orm::ActiveValue::NotSet;
use sea_orm::{ActiveModelTrait, ColumnTrait, Database, DatabaseConnection, DbErr, DeleteOne, EntityTrait, QueryFilter, Set, TransactionTrait};

mod entities;
mod models;

#[tokio::main]
async fn main() {
    server().await;
}

async fn server() {
    let router = Router::new()
    .route("/api/product/getall", get(get_all_products))
    .route("/api/product/create", post(create_product))
    .route("/api/product/:id/update", put(update_product))
    .route("/api/product/:id/delete", delete(delete_product));


    // .route("/api/product/getall", get(get_all_products))
    // .route("/api/product/create", post(create_product))
    // .route("/api/product/:id/update", put(update_product))
    // .route("/api/product/:id/delete", delete(delete_item));



    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

async fn get_all_products() -> impl IntoResponse {
    let db: DatabaseConnection =
        Database::connect("postgresql://postgres:root@localhost:5432/products_db")
            .await
            .unwrap();

    // Fetch all products with related items
    let products_with_items = product::Entity::find()
        .find_with_related(item::Entity)
        .all(&db)
        .await
        .unwrap();

    // Transform the result into a custom response format
    let response: Vec<ProductItemModel> = products_with_items
        .into_iter()
        .map(|(product, items)| ProductItemModel {
            id: product.id,
            name:product.name,
            description: product.description,
            items: items
                .into_iter()
                .map(|item| ItemModel {
                    id: item.id,
                    product_id:item.product_id,
                    color: item.color,
                    size: item.size,
                    stock: item.stock,
                })
                .collect(),
        })
        .collect();

    db.close().await.unwrap();

    (StatusCode::OK, axum::Json(response))
}

// async fn get_product_by_id(
//     Path(product_id): Path<i32>,
// ) -> impl IntoResponse {
//     let db: DatabaseConnection =
//         Database::connect("postgresql://postgres:root@localhost:5432/products_db")
//             .await
//             .unwrap();

//         let product_data = product::Entity::find_by_id(product_id)
//             .find_with_related(item::Entity)
//             .all(&db)
//             .await;

//             if product_data() {
//                 return (StatusCode::NOT_FOUND, Json("Product not found")).into_response();
//             }
        
//             let result: Vec<ProductItemModel> = product_data
//                 .into_iter()
//                 .map(|(product, items)| ProductItemModel {
//                     id: product.id,
//                     name:product.name,
//                     description: product.description,
//                     items: items
//                         .into_iter()
//                         .map(|item| ItemModel {
//                             id: item.id,
//                             product_id:item.product_id,
//                             color: item.color,
//                             size: item.size,
//                             stock: item.stock,
//                         })
//                         .collect(),
//                 })
//                 .collect();
        
//             (StatusCode::OK, Json(result)).into_response()
// }

async fn create_product(
    Json(product_data): Json<CreateProductModal>,
) -> impl IntoResponse {
    let db: DatabaseConnection =
        Database::connect("postgresql://postgres:root@localhost:5432/products_db")
            .await
            .unwrap();

    let now: NaiveDateTime = Utc::now().naive_utc();

    let product_model = product::ActiveModel {
        name: Set(product_data.name.to_owned()),
        description: Set(product_data.description.to_owned()),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };

    let _result = product_model.insert(&db).await.unwrap();

    (StatusCode::CREATED, "Product created")
}


async fn create_item(
    Json(item_data): Json<ItemModel>,
) -> impl IntoResponse {
    let db: DatabaseConnection =
        Database::connect("postgresql://postgres:root@localhost:5432/products_db")
            .await
            .unwrap();

    let item = item::ActiveModel {
        product_id: Set(item_data.product_id.to_owned()),
        color: Set(item_data.color.to_owned()),
        stock: Set(item_data.stock.to_owned()),
        size: Set(item_data.size.to_owned()),
        ..Default::default()
    };

    let _result = item.insert(&db).await.unwrap();

    (StatusCode::CREATED, "Item created")
}


async fn update_product(
    Path(product_id): Path<i32>,
    Json(product_data): Json<UpdateProductModal>,
) -> impl IntoResponse {
    let db: DatabaseConnection =
        Database::connect("postgresql://postgres:root@localhost:5432/products_db")
            .await
            .unwrap();

    let now: NaiveDateTime = Utc::now().naive_utc();

    let mut updated_product:product::ActiveModel = product::Entity::find()
        .filter(product::Column::Id.eq(product_id))
        .one(&db)
        .await
        .unwrap()
        .unwrap().into();
        
        updated_product.name = match product_data.name {
            Some(name) => Set(name),
            None => NotSet,
        };    
    
    updated_product.description=Set(product_data.description);
    updated_product.updated_at = Set(now);

    updated_product.update(&db).await.unwrap();

    db.close().await.unwrap();
    
    (StatusCode::ACCEPTED,"Product updated")
}


async fn delete_item(
    Path(item_id): Path<i32>,
) -> impl IntoResponse {
    let db: DatabaseConnection =
        Database::connect("postgresql://postgres:root@localhost:5432/products_db")
            .await
            .unwrap();

    // Check if the item exists
    let item_exist = item::Entity::find_by_id(item_id)
        .one(&db)
        .await
        .unwrap();


    if item_exist.is_none() {
        // Item not found
        (StatusCode::NOT_FOUND, Json("Item not found")).into_response();
    } else {
        // Delete the item
        item::Entity::delete_by_id(item_id)
            .exec(&db)
            .await
            .unwrap();

        (StatusCode::OK, Json("Item deleted")).into_response();
    }

    db.close().await.unwrap();

}

async fn delete_product(
    Path(product_id): Path<i32>,
) -> impl IntoResponse {
    let db: DatabaseConnection =
        Database::connect("postgresql://postgres:root@localhost:5432/products_db")
            .await
            .unwrap();

    // Check if the product exists
    let product_exist = product::Entity::find_by_id(product_id)
        .one(&db)
        .await
        .unwrap();

    if product_exist.is_none() {
        db.close().await.unwrap();
        return (StatusCode::NOT_FOUND, Json("Product not found")).into_response();
    }

    // Begin a transaction to ensure atomicity
    let txn = db.begin().await.unwrap();

    // Delete associated items first
    item::Entity::delete_many()
        .filter(item::Column::ProductId.eq(product_id))
        .exec(&txn)
        .await
        .unwrap();

    // Delete the product
    product::Entity::delete_by_id(product_id)
        .exec(&txn)
        .await
        .unwrap();

    // Commit the transaction
    txn.commit().await.unwrap();

    db.close().await.unwrap();

    (StatusCode::OK, Json("Product and associated items deleted")).into_response()
}

