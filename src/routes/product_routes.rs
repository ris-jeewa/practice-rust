use axum::{
    routing::{get, post, put, delete}, 
    Router,
    http::Method
};
use tower_http::cors::{Any, CorsLayer};

use crate::{handler::product_handler::{create_product, delete_product, get_all_products, update_product}, services::product_service::ProductService};


pub fn product_routes(product_service:ProductService) -> Router {
    let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT])
    .allow_origin(Any);

    Router::new()
    .route("/product", post(create_product))
    .route("/product", get(get_all_products))
    .route("/product/:id", put(update_product))
    .route("/product/:id", delete(delete_product))
    .layer(cors)
    .with_state(product_service)

}

