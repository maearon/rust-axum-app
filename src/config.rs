use serde::Deserialize;
use crate::error::AppError;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub port: u16,
    pub frontend_url: String,
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub from_email: String,
}

impl Config {
    pub fn from_env() -> Result<Self, AppError> {
        dotenvy::dotenv().ok();
        
        Ok(Self {
            database_url: std::env::var("DATABASE_URL")
                .map_err(|_| AppError::Config("DATABASE_URL must be set".to_string()))?,
            jwt_secret: std::env::var("JWT_SECRET")
                .map_err(|_| AppError::Config("JWT_SECRET must be set".to_string()))?,
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .map_err(|_| AppError::Config("Invalid PORT".to_string()))?,
            frontend_url: std::env::var("FRONTEND_URL")
                .unwrap_or_else(|_| "http://localhost:3000".to_string()),
            smtp_host: std::env::var("SMTP_HOST")
                .unwrap_or_else(|_| "localhost".to_string()),
            smtp_port: std::env::var("SMTP_PORT")
                .unwrap_or_else(|_| "587".to_string())
                .parse()
                .map_err(|_| AppError::Config("Invalid SMTP_PORT".to_string()))?,
            smtp_username: std::env::var("SMTP_USERNAME")
                .unwrap_or_else(|_| "".to_string()),
            smtp_password: std::env::var("SMTP_PASSWORD")
                .unwrap_or_else(|_| "".to_string()),
            from_email: std::env::var("FROM_EMAIL")
                .unwrap_or_else(|_| "noreply@example.com".to_string()),
        })
    }
}
