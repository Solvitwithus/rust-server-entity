use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use sea_orm::{
    ActiveValue::Set,
    ColumnTrait,
    DatabaseConnection,
    EntityTrait,
    QueryFilter,
    QuerySelect,
};

use serde::{Deserialize, Serialize};
use serde_json::json;

use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, Header, EncodingKey};

use dotenvy::dotenv;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::entities::login::{ActiveModel, Entity, Column};



// =====================
// PAYLOADS
// =====================

#[derive(Deserialize)]
pub struct RegisterPayload {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    pub username: String,
    pub company: String,
    pub exp: usize,
}



// =====================
// REGISTER
// =====================

pub async fn register_handler(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<RegisterPayload>,
) -> impl IntoResponse {

    let hashed_password = match hash(payload.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Failed to hash password" })),
            )
        }
    };

    let model = ActiveModel {
        username: Set(payload.username),
        password: Set(hashed_password),
        ..Default::default()
    };

    match model.insert(&db).await {
        Ok(_) => (
            StatusCode::CREATED,
            Json(json!({ "message": "User registered successfully" })),
        ),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to register user",
                "details": err.to_string()
            })),
        ),
    }
}



// =====================
// LOGIN
// =====================

pub async fn login_handler(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<LoginPayload>,
) -> impl IntoResponse {

    dotenv().ok();

    let secret = match env::var("JWT_SECRET") {
        Ok(v) => v,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "JWT_SECRET not set" })),
            )
        }
    };

    let user = match Entity::find()
        .filter(Column::Username.eq(payload.username))
        .one(&db)
        .await
    {
        Ok(Some(user)) => user,
        Ok(None) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "Invalid credentials" })),
            )
        }
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": err.to_string() })),
            )
        }
    };

    match verify(payload.password, &user.password) {
        Ok(true) => (),
        _ => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "Invalid credentials" })),
            )
        }
    };

    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() + 60 * 60; // 1 hour

    let claims = Claims {
        username: user.username,
        company: "Digisoft".to_string(),
        exp: exp as usize,
    };

    let token = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    ) {
        Ok(t) => t,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Failed to generate token" })),
            )
        }
    };

    (
        StatusCode::OK,
        Json(json!({
            "message": "Login successful",
            "token": token
        })),
    )
}



// =====================
// GET USERNAMES ONLY
// =====================

pub async fn login_get_handler(
    State(db): State<DatabaseConnection>,
) -> impl IntoResponse {

    match Entity::find()
        .select_only()
        .column(Column::Username)
        .into_tuple::<String>()
        .all(&db)
        .await
    {
        Ok(usernames) => (
            StatusCode::OK,
            Json(json!({ "usernames": usernames })),
        ),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to fetch usernames",
                "details": err.to_string()
            })),
        ),
    }
}
