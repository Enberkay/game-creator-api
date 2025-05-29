use sea_orm::{Database, DatabaseConnection, DbErr};
use std::env;

pub async fn connect() -> Result<DatabaseConnection, DbErr> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Database::connect(&database_url).await
}
