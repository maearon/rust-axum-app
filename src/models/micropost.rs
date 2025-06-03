use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Micropost {
    pub id: Uuid,
    pub content: String,
    pub user_id: Uuid,
    pub picture: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateMicropostRequest {
    #[validate(length(min = 1, max = 140))]
    pub content: String,
    pub picture: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateMicropostRequest {
    #[validate(length(min = 1, max = 140))]
    pub content: Option<String>,
    pub picture: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MicropostResponse {
    pub id: Uuid,
    pub content: String,
    pub picture: Option<String>,
    pub created_at: DateTime<Utc>,
    pub user: MicropostUserResponse,
}

#[derive(Debug, Serialize)]
pub struct MicropostUserResponse {
    pub id: Uuid,
    pub name: String,
    pub gravatar_url: String,
}

impl Micropost {
    pub fn to_response(&self, user_name: String, user_email: String) -> MicropostResponse {
        let hash = md5::compute(user_email.trim().to_lowercase().as_bytes());
        let gravatar_url = format!("https://www.gravatar.com/avatar/{:x}?s=50&d=identicon", hash);
        
        MicropostResponse {
            id: self.id,
            content: self.content.clone(),
            picture: self.picture.clone(),
            created_at: self.created_at,
            user: MicropostUserResponse {
                id: self.user_id,
                name: user_name,
                gravatar_url,
            },
        }
    }
}
