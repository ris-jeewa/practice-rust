use axum::{http::Method, routing::{get, post,put,delete}, Router};
use tower_http::cors::{Any, CorsLayer};

use crate::handler::product_handler::{create_product, delete_product, get_all_products, update_product};




pub fn product_routes() -> Router {
    let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT])
    .allow_origin(Any);

    let router = Router::new()
    .route("/api/product/getall", get(get_all_products))
    .route("/api/product/create", post(create_product))
    .route("/api/product/:id/update", put(update_product))
    .route("/api/product/:id/delete", delete(delete_product))
    .layer(cors);

    router
}