use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use repositories::{item_repository::ItemRepository, product_repository::ProductRepository};
use routes::{item_routes::item_routes, product_routes::product_routes};
use sea_orm::DatabaseConnection;
use services::{item_service::ItemService, product_service::ProductService};
use utils::db::establish_connection;


mod entities;
mod handler;
mod models;
mod repositories;
mod routes;
mod services;
mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Establish database connection
    let db = establish_connection().await;

    server(db).await;
}

async fn server(db: DatabaseConnection) {
    let product_repository = ProductRepository::new(db.clone());
    let item_repository = ItemRepository::new(db);

    let product_service = ProductService::new(product_repository);

    let item_service = ItemService::new(item_repository);

    let default_route = get(default_handler);

    let router = Router::new()
        .merge(product_routes(product_service))
        .merge(item_routes(item_service))
        .route("/", default_route); 

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}



// Default route handler function
async fn default_handler() -> impl axum::response::IntoResponse {
    let response = r#"
📋 Welcome to the Product-Item Microservice<br><br>
Available API Endpoints:<br>
----------------------------------<br>
<strong>Product Endpoints</strong>:<br> 
🔹 GET /product       - Lists all products.<br>
🔹 POST /product      - Create a new product.<br>
🔹 PUT /product/{id}  - Update a product by ID.<br>
🔹 DELETE /product/{id} - Delete a product by ID.<br><br>

<strong>Item Endpoints</strong>:<br>
🔹 GET /item/{id}         - Get an item by ID.<br>
🔹 POST /item             - Create a new item.<br>
🔹 PUT /item/{id}         - Update an item by ID.<br>
🔹 DELETE /items/{id}     - Delete an item by ID.<br><br>

Happy coding!<br>
"#;
    Html(response) 
}