use sqlx::{PgPool, postgres::PgPoolOptions};
use crate::error::AppError;

pub mod users;
pub mod microposts;
pub mod relationships;

pub async fn init(database_url: &str) -> Result<PgPool, AppError> {
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(database_url)
        .await?;
    
    Ok(pool)
}
