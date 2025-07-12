use actix_web::{web, HttpRequest, HttpResponse, Result};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait, Set};
use serde_json::json;
use uuid::Uuid;

use crate::dtos::auth_dto::{LoginRequest, RegisterRequest, AuthResponse, UserInfo, CurrentUserResponse};
use crate::middleware::auth::Claims;
use crate::models::user;

pub async fn register(
    db: web::Data<DatabaseConnection>,
    req: web::Json<RegisterRequest>,
) -> Result<HttpResponse> {
    let db = db.get_ref();

    // Check if user already exists
    let existing_user = user::Entity::find()
        .filter(user::Column::Email.eq(&req.email))
        .one(db)
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    if existing_user.is_some() {
        return Ok(HttpResponse::Conflict().json(json!({
            "error": "User with this email already exists"
        })));
    }

    // Hash password
    let password_hash = hash(&req.password, DEFAULT_COST)
        .map_err(|e| {
            eprintln!("Password hashing error: {}", e);
            actix_web::error::ErrorInternalServerError("Password hashing error")
        })?;

    // Create new user
    let user_id = Uuid::new_v4();
    let now = Utc::now();

    let new_user = user::ActiveModel {
        id: Set(user_id),
        email: Set(req.email.clone()),
        password_hash: Set(password_hash),
        role: Set(req.role.clone()),
        created_at: Set(now.into()),
        updated_at: Set(now.into()),
    };

    let user = new_user
        .insert(db)
        .await
        .map_err(|e| {
            eprintln!("User creation error: {}", e);
            actix_web::error::ErrorInternalServerError("User creation error")
        })?;

    // Generate JWT token
    let token = generate_jwt(&user)?;

    let response = AuthResponse {
        token,
        user: UserInfo {
            id: user.id,
            email: user.email,
            role: user.role,
        },
    };

    Ok(HttpResponse::Created().json(response))
}

pub async fn login(
    db: web::Data<DatabaseConnection>,
    req: web::Json<LoginRequest>,
) -> Result<HttpResponse> {
    let db = db.get_ref();

    // Find user by email
    let user = user::Entity::find()
        .filter(user::Column::Email.eq(&req.email))
        .one(db)
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?
        .ok_or_else(|| {
            actix_web::error::ErrorUnauthorized("Invalid email or password")
        })?;

    // Verify password
    let is_valid = verify(&req.password, &user.password_hash)
        .map_err(|e| {
            eprintln!("Password verification error: {}", e);
            actix_web::error::ErrorInternalServerError("Password verification error")
        })?;

    if !is_valid {
        return Err(actix_web::error::ErrorUnauthorized("Invalid email or password"));
    }

    // Generate JWT token
    let token = generate_jwt(&user)?;

    let response = AuthResponse {
        token,
        user: UserInfo {
            id: user.id,
            email: user.email,
            role: user.role,
        },
    };

    Ok(HttpResponse::Ok().json(response))
}

pub async fn current_user(
    req: HttpRequest,
    db: web::Data<DatabaseConnection>,
) -> Result<HttpResponse> {
    let db = db.get_ref();

    // Function 3: Get current user from database using req.user
    let user = crate::middleware::auth::get_current_user(&req, db).await?;

    let response = CurrentUserResponse {
        id: user.id,
        email: user.email,
        role: user.role,
    };

    Ok(HttpResponse::Ok().json(response))
}

fn generate_jwt(user: &user::Model) -> Result<String, actix_web::Error> {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user.id.to_string(),
        email: user.email.clone(),
        role: user.role.clone(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|e| {
        eprintln!("JWT encoding error: {}", e);
        actix_web::error::ErrorInternalServerError("JWT encoding error")
    })
} 