use axum::{http::Method, routing::{get, post,put,delete}, Router};
use tower_http::cors::{Any, CorsLayer};

use crate::handler::item_handler::{create_item, delete_item, get_item_by_id, update_item};


pub fn item_routes() -> Router {
    let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT])
    .allow_origin(Any);

    let router = Router::new()
    .route("/api/item/:id", get(get_item_by_id))
    .route("/api/item/create", post(create_item))
    .route("/api/item/:id/update", put(update_item))
    .route("/api/item/:id/delete", delete(delete_item))
    .layer(cors);

    router
}