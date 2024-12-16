use sea_orm::{Database, DatabaseConnection};

use super::constants::DATABASE_URL;

pub async fn establish_connection() -> DatabaseConnection {
    let conn_str = (DATABASE_URL).clone();
        Database::connect(conn_str)
            .await
            .expect("Failed to connect to db")

}