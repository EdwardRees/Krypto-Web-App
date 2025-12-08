use dotenv::dotenv;
use sea_orm::{Database, DatabaseConnection};
use std::env;

fn read_env(key: &str, error: &str) -> Result<String, sea_orm::error::DbErr>{
    let value = match env::var(key) {
        Ok(value) => value,
        Err(_) => {
            return Err(sea_orm::error::DbErr::Custom(String::from(error)));
        }
    };

    Ok(value)
}

pub async fn establish_connection() -> Result<DatabaseConnection, sea_orm::error::DbErr>{
    let _ = dotenv().ok();

    let username = read_env("DB_USERNAME", "No username found!")?;

    let password = read_env("DB_PASSWORD", "No password found!")?;
    let host= read_env("DB_HOST", "No database host found!")?;
    let database = read_env("DB_DATABASE", "No database found!")?;

    let db: DatabaseConnection = Database::connect(format!("postgresql://{}:{}@{}/{}", username, password, host, database)).await?;

    Ok(db)
}
