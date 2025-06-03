use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password_digest: String,
    pub admin: bool,
    pub activated: bool,
    pub activated_at: Option<DateTime<Utc>>,
    pub activation_digest: Option<String>,
    pub reset_digest: Option<String>,
    pub reset_sent_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 1, max = 50))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
    pub password_confirmation: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(length(min = 1, max = 50))]
    pub name: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(length(min = 6))]
    pub password: Option<String>,
    pub password_confirmation: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
    pub remember_me: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub admin: bool,
    pub activated: bool,
    pub created_at: DateTime<Utc>,
    pub gravatar_url: String,
    pub microposts_count: i64,
    pub following_count: i64,
    pub followers_count: i64,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub user: UserResponse,
    pub token: String,
    pub refresh_token: String,
}

impl User {
    pub fn gravatar_url(&self, size: u32) -> String {
        let hash = md5::compute(self.email.trim().to_lowercase().as_bytes());
        format!("https://www.gravatar.com/avatar/{:x}?s={}&d=identicon", hash, size)
    }
    
    pub fn to_response(&self) -> UserResponse {
        UserResponse {
            id: self.id,
            name: self.name.clone(),
            email: self.email.clone(),
            admin: self.admin,
            activated: self.activated,
            created_at: self.created_at,
            gravatar_url: self.gravatar_url(80),
            microposts_count: 0, // Will be populated by service
            following_count: 0,  // Will be populated by service
            followers_count: 0,  // Will be populated by service
        }
    }
}
