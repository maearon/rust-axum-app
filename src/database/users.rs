use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use crate::models::{User, CreateUserRequest, UpdateUserRequest};
use crate::error::AppError;
use crate::utils::pagination::{Pagination, PaginationParams};

pub struct UserRepository;

impl UserRepository {
    pub async fn create(pool: &PgPool, req: &CreateUserRequest, password_hash: &str, activation_digest: &str) -> Result<User, AppError> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, name, email, password_digest, activation_digest, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
            Uuid::new_v4(),
            req.name,
            req.email.to_lowercase(),
            password_hash,
            activation_digest,
            Utc::now(),
            Utc::now()
        )
        .fetch_one(pool)
        .await?;
        
        Ok(user)
    }
    
    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE id = $1",
            id
        )
        .fetch_optional(pool)
        .await?;
        
        Ok(user)
    }
    
    pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE email = $1",
            email.to_lowercase()
        )
        .fetch_optional(pool)
        .await?;
        
        Ok(user)
    }
    
    pub async fn find_by_activation_token(pool: &PgPool, token: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE activation_digest = $1",
            token
        )
        .fetch_optional(pool)
        .await?;
        
        Ok(user)
    }
    
    pub async fn find_by_reset_token(pool: &PgPool, token: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE reset_digest = $1 AND reset_sent_at > NOW() - INTERVAL '2 hours'",
            token
        )
        .fetch_optional(pool)
        .await?;
        
        Ok(user)
    }
    
    pub async fn update(pool: &PgPool, id: Uuid, req: &UpdateUserRequest, password_hash: Option<&str>) -> Result<User, AppError> {
        let mut query = "UPDATE users SET updated_at = $1".to_string();
        let mut param_count = 1;
        let mut params: Vec<Box<dyn sqlx::Encode<'_, sqlx::Postgres> + Send + Sync>> = vec![Box::new(Utc::now())];
        
        if let Some(name) = &req.name {
            param_count += 1;
            query.push_str(&format!(", name = ${}", param_count));
            params.push(Box::new(name.clone()));
        }
        
        if let Some(email) = &req.email {
            param_count += 1;
            query.push_str(&format!(", email = ${}", param_count));
            params.push(Box::new(email.to_lowercase()));
        }
        
        if let Some(hash) = password_hash {
            param_count += 1;
            query.push_str(&format!(", password_digest = ${}", param_count));
            params.push(Box::new(hash.to_string()));
        }
        
        param_count += 1;
        query.push_str(&format!(" WHERE id = ${} RETURNING *", param_count));
        params.push(Box::new(id));
        
        // For simplicity, we'll use a direct query here
        // In a real application, you might want to use a query builder
        let user = if let Some(name) = &req.name {
            if let Some(email) = &req.email {
                if let Some(hash) = password_hash {
                    sqlx::query_as!(
                        User,
                        "UPDATE users SET name = $1, email = $2, password_digest = $3, updated_at = $4 WHERE id = $5 RETURNING *",
                        name,
                        email.to_lowercase(),
                        hash,
                        Utc::now(),
                        id
                    )
                    .fetch_one(pool)
                    .await?
                } else {
                    sqlx::query_as!(
                        User,
                        "UPDATE users SET name = $1, email = $2, updated_at = $3 WHERE id = $4 RETURNING *",
                        name,
                        email.to_lowercase(),
                        Utc::now(),
                        id
                    )
                    .fetch_one(pool)
                    .await?
                }
            } else if let Some(hash) = password_hash {
                sqlx::query_as!(
                    User,
                    "UPDATE users SET name = $1, password_digest = $2, updated_at = $3 WHERE id = $4 RETURNING *",
                    name,
                    hash,
                    Utc::now(),
                    id
                )
                .fetch_one(pool)
                .await?
            } else {
                sqlx::query_as!(
                    User,
                    "UPDATE users SET name = $1, updated_at = $2 WHERE id = $3 RETURNING *",
                    name,
                    Utc::now(),
                    id
                )
                .fetch_one(pool)
                .await?
            }
        } else if let Some(email) = &req.email {
            if let Some(hash) = password_hash {
                sqlx::query_as!(
                    User,
                    "UPDATE users SET email = $1, password_digest = $2, updated_at = $3 WHERE id = $4 RETURNING *",
                    email.to_lowercase(),
                    hash,
                    Utc::now(),
                    id
                )
                .fetch_one(pool)
                .await?
            } else {
                sqlx::query_as!(
                    User,
                    "UPDATE users SET email = $1, updated_at = $2 WHERE id = $3 RETURNING *",
                    email.to_lowercase(),
                    Utc::now(),
                    id
                )
                .fetch_one(pool)
                .await?
            }
        } else if let Some(hash) = password_hash {
            sqlx::query_as!(
                User,
                "UPDATE users SET password_digest = $1, updated_at = $2 WHERE id = $3 RETURNING *",
                hash,
                Utc::now(),
                id
            )
            .fetch_one(pool)
            .await?
        } else {
            return Err(AppError::BadRequest("No fields to update".to_string()));
        };
        
        Ok(user)
    }
    
    pub async fn activate(pool: &PgPool, id: Uuid) -> Result<User, AppError> {
        let user = sqlx::query_as!(
            User,
            "UPDATE users SET activated = true, activated_at = $1, activation_digest = NULL, updated_at = $2 WHERE id = $3 RETURNING *",
            Utc::now(),
            Utc::now(),
            id
        )
        .fetch_one(pool)
        .await?;
        
        Ok(user)
    }
    
    pub async fn set_reset_token(pool: &PgPool, id: Uuid, reset_digest: &str) -> Result<User, AppError> {
        let user = sqlx::query_as!(
            User,
            "UPDATE users SET reset_digest = $1, reset_sent_at = $2, updated_at = $3 WHERE id = $4 RETURNING *",
            reset_digest,
            Utc::now(),
            Utc::now(),
            id
        )
        .fetch_one(pool)
        .await?;
        
        Ok(user)
    }
    
    pub async fn clear_reset_token(pool: &PgPool, id: Uuid) -> Result<User, AppError> {
        let user = sqlx::query_as!(
            User,
            "UPDATE users SET reset_digest = NULL, reset_sent_at = NULL, updated_at = $1 WHERE id = $2 RETURNING *",
            Utc::now(),
            id
        )
        .fetch_one(pool)
        .await?;
        
        Ok(user)
    }
    
    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
        sqlx::query!("DELETE FROM users WHERE id = $1", id)
            .execute(pool)
            .await?;
        
        Ok(())
    }
    
    pub async fn list(pool: &PgPool, params: &PaginationParams) -> Result<Pagination<User>, AppError> {
        let offset = (params.page - 1) * params.per_page;
        
        let users = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE activated = true ORDER BY created_at DESC LIMIT $1 OFFSET $2",
            params.per_page as i64,
            offset as i64
        )
        .fetch_all(pool)
        .await?;
        
        let total = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM users WHERE activated = true"
        )
        .fetch_one(pool)
        .await?
        .unwrap_or(0) as usize;
        
        Ok(Pagination::new(users, total, params.page, params.per_page))
    }
    
    pub async fn get_stats(pool: &PgPool, user_id: Uuid) -> Result<(i64, i64, i64), AppError> {
        let microposts_count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM microposts WHERE user_id = $1",
            user_id
        )
        .fetch_one(pool)
        .await?
        .unwrap_or(0);
        
        let following_count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM relationships WHERE follower_id = $1",
            user_id
        )
        .fetch_one(pool)
        .await?
        .unwrap_or(0);
        
        let followers_count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM relationships WHERE followed_id = $1",
            user_id
        )
        .fetch_one(pool)
        .await?
        .unwrap_or(0);
        
        Ok((microposts_count, following_count, followers_count))
    }
}
