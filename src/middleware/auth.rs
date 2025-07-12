use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    http::header,
    Error, HttpMessage, HttpRequest,
};
use futures_util::future::{ready, LocalBoxFuture, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use serde::{Deserialize, Serialize};
use std::env;
use std::rc::Rc;
use uuid::Uuid;

use crate::models::user;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // user id
    pub email: String,
    pub role: String,
    pub exp: usize,
}

#[derive(Clone)]
pub struct AuthMiddleware {
    pub required_role: Option<String>,
}

impl AuthMiddleware {
    pub fn new() -> Self {
        Self { required_role: None }
    }

    pub fn with_role(role: String) -> Self {
        Self {
            required_role: Some(role),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service: Rc::new(service),
            required_role: self.required_role.clone(),
        }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
    required_role: Option<String>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        let required_role = self.required_role.clone();

        Box::pin(async move {
            // Function 1: Decode JWT and attach to req.user
            let token = extract_token_from_request(&req);
            if token.is_none() {
                return Err(ErrorUnauthorized("No token provided"));
            }

            let token = token.unwrap();
            let claims = decode_jwt(&token)?;
            
            // Attach user info to request extensions
            req.extensions_mut().insert(claims.clone());

            // Function 2: Check role if required
            if let Some(required_role) = required_role {
                if claims.role != required_role {
                    return Err(ErrorUnauthorized("Insufficient permissions"));
                }
            }

            let res = service.call(req).await?;
            Ok(res)
        })
    }
}

// Function 1: Extract and decode JWT token
fn extract_token_from_request(req: &ServiceRequest) -> Option<String> {
    req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_str| {
            if auth_str.starts_with("Bearer ") {
                Some(auth_str[7..].to_string())
            } else {
                None
            }
        })
}

fn decode_jwt(token: &str) -> Result<Claims, Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let key = DecodingKey::from_secret(secret.as_ref());
    
    let token_data = decode::<Claims>(token, &key, &Validation::default())
        .map_err(|_| ErrorUnauthorized("Invalid token"))?;
    
    Ok(token_data.claims)
}

// Function 3: Get current user from database
pub async fn get_current_user(
    req: &HttpRequest,
    db: &DatabaseConnection,
) -> Result<user::Model, Error> {
    let claims = req
        .extensions()
        .get::<Claims>()
        .ok_or_else(|| ErrorUnauthorized("User not found in request"))?
        .clone();

    let user_id: Uuid = claims.sub.parse().map_err(|_| ErrorUnauthorized("Invalid user ID"))?;

    let user = user::Entity::find_by_id(user_id)
        .filter(user::Column::Id.eq(user_id))
        .one(db)
        .await
        .map_err(|_| ErrorUnauthorized("Database error"))?
        .ok_or_else(|| ErrorUnauthorized("User not found"))?;

    Ok(user)
}

// Helper function to extract user from request
pub fn get_user_from_request(req: &HttpRequest) -> Result<Claims, Error> {
    req.extensions()
        .get::<Claims>()
        .cloned()
        .ok_or_else(|| ErrorUnauthorized("User not found in request"))
} 