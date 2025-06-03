use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID
    pub exp: i64,    // Expiration time
    pub iat: i64,    // Issued at
    pub token_type: String, // "access" or "refresh"
}

pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtService {
    pub fn new(secret: &str) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_ref()),
            decoding_key: DecodingKey::from_secret(secret.as_ref()),
        }
    }
    
    pub fn generate_access_token(&self, user_id: Uuid) -> Result<String, AppError> {
        let now = Utc::now();
        let claims = Claims {
            sub: user_id.to_string(),
            exp: (now + Duration::hours(1)).timestamp(),
            iat: now.timestamp(),
            token_type: "access".to_string(),
        };
        
        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AppError::Internal(format!("Failed to generate token: {}", e)))
    }
    
    pub fn generate_refresh_token(&self, user_id: Uuid) -> Result<String, AppError> {
        let now = Utc::now();
        let claims = Claims {
            sub: user_id.to_string(),
            exp: (now + Duration::days(30)).timestamp(),
            iat: now.timestamp(),
            token_type: "refresh".to_string(),
        };
        
        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AppError::Internal(format!("Failed to generate refresh token: {}", e)))
    }
    
    pub fn verify_token(&self, token: &str) -> Result<Claims, AppError> {
        decode::<Claims>(token, &self.decoding_key, &Validation::default())
            .map(|data| data.claims)
            .map_err(|_| AppError::Unauthorized("Invalid token".to_string()))
    }
}
