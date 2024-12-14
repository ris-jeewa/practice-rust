use axum:: Router;
use repositories::product_repository::ProductRepository;
use routes::product_routes::product_routes;
use sea_orm::Database;
use services::product_service::ProductService;

mod entities;
mod models;
mod routes;
mod handler;
mod utils;
mod services;
mod repositories;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    server().await;
}

async fn server() {

    let conn_str = (*utils::constants::DATABASE_URL).clone();
    let db = Database::connect(conn_str).await.expect("Failed to connect to db");

    // let router = Router::new()
    // .merge(routes::product_routes::product_routes())
    // .merge(routes::item_routes::item_routes())
    // .layer(Extension(db));
    let product_repository = ProductRepository::new(db);
    let product_service = ProductService::new(product_repository);

    let router = Router::new()
    .merge(product_routes(product_service));
    
 
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}