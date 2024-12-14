use axum::{
    routing::{get, post, put, delete}, 
    Router,
    http::Method
};
use tower_http::cors::{Any, CorsLayer};

// use crate::{handler::product_handler::create_product};
use crate::{handler::product_handler::create_product, services::product_service::ProductService};


pub fn product_routes(product_service:ProductService) -> Router {
    let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT])
    .allow_origin(Any);

    Router::new()
    .route("/product", post(create_product))
    // .route("/product", get(get_all_products))
    // .route("/api/product/:id/update", put(update_product))
    // .route("/api/product/:id/delete", delete(delete_product))
    .layer(cors)
    .with_state(product_service)

}

