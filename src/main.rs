mod auth;
mod models;

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use models::{AuthResponse, LoginRequest, RegisterRequest, User};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("failed to connect to postgres");

    println!("connected to Postgres");

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();

    println!("Listening on 0.0.0.0:8080");
    axum::serve(listener, app).await.unwrap();
}

async fn register(
    State(pool): State<PgPool>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<User>, (StatusCode, String)> {
    let password_hash = auth::hash_password(&req.password)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (email, password_hash, display_name)
         VALUES ($1, $2, $3)
         RETURNING id, email, password_hash, display_name, created_at",
    )
    .bind(&req.email)
    .bind(&password_hash)
    .bind(&req.display_name)
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::CONFLICT, format!("could not create user: {e}")))?;

    Ok(Json(user))
}

async fn login(
    State(pool): State<PgPool>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let user = sqlx::query_as::<_, User>(
        "SELECT id, email, password_hash, display_name, created_at
         FROM users WHERE email = $1",
    )
    .bind(&req.email)
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or((StatusCode::UNAUTHORIZED, "invalid credentials".to_string()))?;

    if !auth::verify_password(&req.password, &user.password_hash) {
        return Err((StatusCode::UNAUTHORIZED, "invalid credentials".to_string()));
    }

    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token = auth::make_token(user.id, &secret)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(AuthResponse {
        token,
        user_id: user.id,
    }))
}