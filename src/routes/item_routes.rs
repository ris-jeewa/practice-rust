use axum::{
    routing::{get, post, put, delete}, 
    Router,
    http::Method
};
use tower_http::cors::{Any, CorsLayer};

use crate::{handler::item_handler::create_item, services::item_service::ItemService};


pub fn item_routes(item_service:ItemService) -> Router {
    let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT])
    .allow_origin(Any);

    Router::new()
    // .route("/item/:id", get(get_item_by_id))
    .route("/item", post(create_item))
    // .route("/item/:id", put(update_item))
    // .route("/item/:id", delete(delete_item))
    .layer(cors)
    .with_state(item_service)
    
}
