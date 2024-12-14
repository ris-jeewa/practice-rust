use axum::{http::{request, StatusCode}, response::IntoResponse};
use chrono::{NaiveDateTime, Utc};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

use crate::{entities::product, models::{product_model::{CreateProductModal, WholeProductModel}, ErrorModel}};

#[derive(Clone)]
pub struct ProductRepository {
    db:DatabaseConnection
}

impl ProductRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db
        }
    }

    pub async fn create_product_in_db(&self ,request:CreateProductModal) -> Result<WholeProductModel,ErrorModel> {
        let now: NaiveDateTime = Utc::now().naive_utc();
    
        let product_model = product::ActiveModel {
            name: Set(request.name.to_owned()),
            description: Set(request.description.to_owned()),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };

        match product_model.insert(&self.db).await {
            Ok(inserted_product) => {
                Ok(WholeProductModel{
                    id: inserted_product.id,
                    name: inserted_product.name,
                    description: inserted_product.description,
                    created_at: inserted_product.created_at,
                    updated_at: inserted_product.updated_at,
                })
                
            }
            Err(_) => {
                Err(ErrorModel::DatabaseError("Failed to create product".to_string()))
            }
        }
    
    }
    
}

