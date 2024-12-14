pub mod item_model;
pub mod product_model;

#[derive(Debug)]
pub enum ErrorModel {
    ValidationError(String),
    DatabaseError(String),
}