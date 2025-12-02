use dotenv::dotenv;
use sea_orm::{Database, DatabaseConnection, DbErr};
use std::env;

fn read_env(key: &str, error: &str) -> Result<String, DbErr>{
    let value = match env::var(key) {
        Ok(value) => value,
        Err(_) => {
            return Err(DbErr::Custom(String::from(error)));
        }
    };

    Ok(value)
}

pub async fn connect_db() -> Result<DatabaseConnection, DbErr>{
    let _ = dotenv();

    let username = read_env("DB_USERNAME", "No username found!")?;

    let password = read_env("DB_PASSWORD", "No password found!")?;
    let host= read_env("DB_HOST", "No database host found!")?;
    let database = read_env("DB_DATABASE", "No database found!")?;

    let db: DatabaseConnection = Database::connect(format!("postgres://{}:{}@{}/{}", username, password, host, database)).await?;

    Ok(db)
}
