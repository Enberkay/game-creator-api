use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::models::creator;
use crate::models::creator::Entity as CreatorEntity;
use crate::models::game::Entity as GameEntity;

use crate::dtos::{CreateCreator, UpdateCreator};

pub async fn create_creator(
    db: web::Data<DatabaseConnection>,
    json: web::Json<CreateCreator>,
) -> impl Responder {
    let new_creator = creator::ActiveModel {
        id: Set(Uuid::new_v4()),
        first_name: Set(json.first_name.clone()),
        last_name: Set(json.last_name.clone()),
        email: Set(json.email.clone()),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
    };

    match new_creator.insert(db.get_ref()).await {
        Ok(creator) => HttpResponse::Created().json(creator),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_all_creators(db: web::Data<DatabaseConnection>) -> impl Responder {
    match CreatorEntity::find().all(db.get_ref()).await {
        Ok(creators) => HttpResponse::Ok().json(creators),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_creator_by_id(
    db: web::Data<DatabaseConnection>,
    path: web::Path<Uuid>,
) -> impl Responder {
    match CreatorEntity::find_by_id(path.into_inner())
        .one(db.get_ref())
        .await
    {
        Ok(Some(creator)) => HttpResponse::Ok().json(creator),
        Ok(None) => HttpResponse::NotFound().body("Creator not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn update_creator(
    db: web::Data<DatabaseConnection>,
    path: web::Path<Uuid>,
    json: web::Json<UpdateCreator>,
) -> impl Responder {
    let id = path.into_inner();
    match CreatorEntity::find_by_id(id).one(db.get_ref()).await {
        Ok(Some(model)) => {
            let mut active_model: creator::ActiveModel = model.into();

            if let Some(first_name) = &json.first_name {
                active_model.first_name = Set(first_name.clone());
            }
            if let Some(last_name) = &json.last_name {
                active_model.last_name = Set(last_name.clone());
            }
            if let Some(email) = &json.email {
                active_model.email = Set(email.clone());
            }

            active_model.updated_at = Set(Utc::now());

            match active_model.update(db.get_ref()).await {
                Ok(updated) => HttpResponse::Ok().json(updated),
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
        Ok(None) => HttpResponse::NotFound().body("Creator not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn delete_creator(
    db: web::Data<DatabaseConnection>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let id = path.into_inner();
    match CreatorEntity::find_by_id(id).one(db.get_ref()).await {
        Ok(Some(model)) => {
            let active_model: creator::ActiveModel = model.into();
            match active_model.delete(db.get_ref()).await {
                Ok(_) => HttpResponse::NoContent().finish(),
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
        Ok(None) => HttpResponse::NotFound().body("Creator not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_games_by_creator(
    db: web::Data<DatabaseConnection>,
    path: web::Path<Uuid>,
) -> impl Responder {
    match GameEntity::find()
        .filter(crate::models::game::Column::CreatorId.eq(path.into_inner()))
        .all(db.get_ref())
        .await
    {
        Ok(games) => HttpResponse::Ok().json(games),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
