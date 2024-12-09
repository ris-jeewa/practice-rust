use axum::extract::Path;
use axum::{Extension, Json};
use axum::{http::StatusCode, response::IntoResponse, Router};
use entities::item;
use models::item_model::ItemModel;
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection,EntityTrait,  Set, };

mod entities;
mod models;
mod routes;
mod handler;
mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    server().await;
}

async fn server() {

    let conn_str = (*utils::constants::DATABASE_URL).clone();
    let db = Database::connect(conn_str).await.expect("Failed to connect to db");

    let router = Router::new()
    .merge(routes::product_routes::product_routes())
    .merge(routes::item_routes::item_routes())
    .layer(Extension(db));
 
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
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









