use axum::{
    extract::{Request, State},
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::Response,
};
use crate::{app::AppState, auth::JwtService, database::users::UserRepository, error::AppError};

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "));
    
    if let Some(token) = auth_header {
        let jwt_service = JwtService::new(&state.config.jwt_secret);
        
        match jwt_service.verify_token(token) {
            Ok(claims) => {
                if claims.token_type == "access" {
                    let user_id = claims.sub.parse().map_err(|_| AppError::Unauthorized("Invalid user ID".to_string()))?;
                    
                    if let Ok(Some(user)) = UserRepository::find_by_id(&state.db, user_id).await {
                        if user.activated {
                            request.extensions_mut().insert(user);
                        } else {
                            return Err(AppError::Unauthorized("Account not activated".to_string()));
                        }
                    } else {
                        return Err(AppError::Unauthorized("User not found".to_string()));
                    }
                } else {
                    return Err(AppError::Unauthorized("Invalid token type".to_string()));
                }
            }
            Err(_) => return Err(AppError::Unauthorized("Invalid token".to_string())),
        }
    } else {
        return Err(AppError::Unauthorized("Missing authorization header".to_string()));
    }
    
    Ok(next.run(request).await)
}

pub async fn optional_auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Response {
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "));
    
    if let Some(token) = auth_header {
        let jwt_service = JwtService::new(&state.config.jwt_secret);
        
        if let Ok(claims) = jwt_service.verify_token(token) {
            if claims.token_type == "access" {
                if let Ok(user_id) = claims.sub.parse() {
                    if let Ok(Some(user)) = UserRepository::find_by_id(&state.db, user_id).await {
                        if user.activated {
                            request.extensions_mut().insert(user);
                        }
                    }
                }
            }
        }
    }
    
    next.run(request).await
}

pub async fn admin_middleware(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if let Some(user) = request.extensions().get::<crate::models::User>() {
        if user.admin {
            Ok(next.run(request).await)
        } else {
            Err(StatusCode::FORBIDDEN)
        }
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
