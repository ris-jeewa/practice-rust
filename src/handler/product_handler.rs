use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use chrono::{NaiveDateTime, Utc};
use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, Database, DatabaseConnection, EntityTrait, QueryFilter, Set, TransactionTrait};

use crate::{entities::{item, product}, models::{item_model::ItemModel, product_model::{CreateProductModal, ProductItemModel, UpdateProductModal}}};



pub async fn get_all_products(
    Extension(db): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    // let db: DatabaseConnection =
    //     Database::connect("postgresql://postgres:root@localhost:5432/products_db")
    //         .await
    //         .unwrap();

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


pub async fn create_product(
    Extension(db): Extension<DatabaseConnection>,

    Json(product_data): Json<CreateProductModal>,
) -> impl IntoResponse {
    // let db: DatabaseConnection =
    //     Database::connect("postgresql://postgres:root@localhost:5432/products_db")
    //         .await
    //         .unwrap();

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

pub async fn update_product(
    Extension(db): Extension<DatabaseConnection>,
    Path(product_id): Path<i32>,
    Json(product_data): Json<UpdateProductModal>,
) -> impl IntoResponse {
    // let db: DatabaseConnection =
    //     Database::connect("postgresql://postgres:root@localhost:5432/products_db")
    //         .await
    //         .unwrap();

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


pub async fn delete_product(
    Extension(db): Extension<DatabaseConnection>,
    Path(product_id): Path<i32>,
) -> impl IntoResponse {
    // let db: DatabaseConnection =
    //     Database::connect("postgresql://postgres:root@localhost:5432/products_db")
    //         .await
    //         .unwrap();

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
