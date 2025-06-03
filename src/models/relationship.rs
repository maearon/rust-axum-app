use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Relationship {
    pub id: Uuid,
    pub follower_id: Uuid,
    pub followed_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateRelationshipRequest {
    pub followed_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct RelationshipResponse {
    pub id: Uuid,
    pub follower_id: Uuid,
    pub followed_id: Uuid,
    pub created_at: DateTime<Utc>,
}

impl Relationship {
    pub fn to_response(&self) -> RelationshipResponse {
        RelationshipResponse {
            id: self.id,
            follower_id: self.follower_id,
            followed_id: self.followed_id,
            created_at: self.created_at,
        }
    }
}
