use axum::{
    extract::DefaultBodyLimit,
    http::{header, Method},
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod app;
mod auth;
mod config;
mod database;
mod error;
mod handlers;
mod models;
mod templates;
mod utils;

use app::AppState;
use config::Config;
use error::AppError;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_boilerplate=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env()?;
    
    // Initialize database
    let pool = database::init(&config.database_url).await?;
    
    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;
    
    // Create app state
    let state = AppState::new(pool, config.clone());

    // Build our application with routes
    let app = create_app(state);

    // Run the server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("Server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

fn create_app(state: AppState) -> Router {
    Router::new()
        // Static pages
        .route("/", get(handlers::static_pages::home))
        .route("/about", get(handlers::static_pages::about))
        .route("/help", get(handlers::static_pages::help))
        .route("/contact", get(handlers::static_pages::contact))
        
        // Authentication routes
        .route("/signup", get(handlers::auth::signup_form).post(handlers::auth::signup))
        .route("/login", get(handlers::auth::login_form).post(handlers::auth::login))
        .route("/logout", post(handlers::auth::logout))
        
        // User routes
        .route("/users", get(handlers::users::index))
        .route("/users/:id", get(handlers::users::show))
        .route("/users/:id/edit", get(handlers::users::edit_form).post(handlers::users::update))
        .route("/users/:id/following", get(handlers::users::following))
        .route("/users/:id/followers", get(handlers::users::followers))
        
        // Micropost routes
        .route("/microposts", post(handlers::microposts::create))
        .route("/microposts/:id", post(handlers::microposts::delete))
        
        // Relationship routes
        .route("/relationships", post(handlers::relationships::create))
        .route("/relationships/:id", post(handlers::relationships::delete))
        
        // Account activation
        .route("/account_activations/:token", get(handlers::auth::activate_account))
        .route("/account_activations/new", get(handlers::auth::activation_form).post(handlers::auth::resend_activation))
        
        // Password reset
        .route("/password_resets/new", get(handlers::auth::password_reset_form).post(handlers::auth::send_password_reset))
        .route("/password_resets/:token", get(handlers::auth::reset_password_form).post(handlers::auth::reset_password))
        
        // API routes
        .nest("/api/v1", create_api_routes())
        
        // Static file serving
        .nest_service("/assets", ServeDir::new("assets"))
        
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(
                    CorsLayer::new()
                        .allow_origin(Any)
                        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
                        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]),
                )
                .layer(DefaultBodyLimit::max(10 * 1024 * 1024)) // 10MB
        )
        .with_state(state)
}

fn create_api_routes() -> Router<AppState> {
    Router::new()
        // API Authentication
        .route("/auth/login", post(handlers::api::auth::login))
        .route("/auth/signup", post(handlers::api::auth::signup))
        .route("/auth/refresh", post(handlers::api::auth::refresh))
        .route("/auth/revoke", post(handlers::api::auth::revoke))
        
        // API Users
        .route("/users", get(handlers::api::users::index).post(handlers::api::users::create))
        .route("/users/:id", get(handlers::api::users::show).put(handlers::api::users::update).delete(handlers::api::users::delete))
        .route("/users/:id/following", get(handlers::api::users::following))
        .route("/users/:id/followers", get(handlers::api::users::followers))
        
        // API Microposts
        .route("/microposts", get(handlers::api::microposts::index).post(handlers::api::microposts::create))
        .route("/microposts/:id", get(handlers::api::microposts::show).put(handlers::api::microposts::update).delete(handlers::api::microposts::delete))
        .route("/feed", get(handlers::api::microposts::feed))
        
        // API Relationships
        .route("/relationships", post(handlers::api::relationships::create))
        .route("/relationships/:id", delete(handlers::api::relationships::delete))
        
        // API Account activation
        .route("/account_activations", post(handlers::api::auth::resend_activation))
        .route("/account_activations/:token", post(handlers::api::auth::activate_account))
        
        // API Password reset
        .route("/password_resets", post(handlers::api::auth::send_password_reset))
        .route("/password_resets/:token", post(handlers::api::auth::reset_password))
}
