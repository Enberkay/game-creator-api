use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;

use crate::models::creator::Entity as CreatorEntity;
use crate::models::game;
use crate::models::game::Entity as GameEntity;

use crate::dtos::{CreateGame, UpdateGame};

pub async fn create_game(
    db: web::Data<DatabaseConnection>,
    json: web::Json<CreateGame>,
) -> impl Responder {
    let new_game = game::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(json.name.clone()),
        description: Set(json.description.clone()),
        genre: Set(json.genre.clone()),
        creator_id: Set(json.creator_id),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
    };

    match new_game.insert(db.get_ref()).await {
        Ok(game) => HttpResponse::Created().json(game),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn list_games(db: web::Data<DatabaseConnection>) -> impl Responder {
    match GameEntity::find().all(db.get_ref()).await {
        Ok(games) => HttpResponse::Ok().json(games),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_game(db: web::Data<DatabaseConnection>, path: web::Path<Uuid>) -> impl Responder {
    let id = path.into_inner();
    match GameEntity::find_by_id(id).one(db.get_ref()).await {
        Ok(Some(game)) => HttpResponse::Ok().json(game),
        Ok(None) => HttpResponse::NotFound().body("Game not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn update_game(
    db: web::Data<DatabaseConnection>,
    path: web::Path<Uuid>,
    json: web::Json<UpdateGame>,
) -> impl Responder {
    let game_id = path.into_inner();
    match GameEntity::find_by_id(game_id).one(db.get_ref()).await {
        Ok(Some(model)) => {
            let mut active_model: game::ActiveModel = model.into();

            if let Some(name) = &json.name {
                active_model.name = Set(name.clone());
            }
            if let Some(description) = &json.description {
                active_model.description = Set(description.clone());
            }
            if let Some(genre) = &json.genre {
                active_model.genre = Set(genre.clone());
            }
            if let Some(creator_id) = json.creator_id {
                active_model.creator_id = Set(creator_id);
            }

            active_model.updated_at = Set(Utc::now());

            match active_model.update(db.get_ref()).await {
                Ok(updated) => HttpResponse::Ok().json(updated),
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
        Ok(None) => HttpResponse::NotFound().body("Game not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn delete_game(
    db: web::Data<DatabaseConnection>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let game_id = path.into_inner();
    match GameEntity::find_by_id(game_id).one(db.get_ref()).await {
        Ok(Some(model)) => {
            let active_model: game::ActiveModel = model.into();
            match active_model.delete(db.get_ref()).await {
                Ok(_) => HttpResponse::NoContent().finish(),
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
        Ok(None) => HttpResponse::NotFound().body("Game not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_game_with_creator(
    db: web::Data<DatabaseConnection>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let game_id = path.into_inner();

    match GameEntity::find_by_id(game_id)
        .find_also_related(CreatorEntity)
        .one(db.get_ref())
        .await
    {
        Ok(Some((game, Some(creator)))) => {
            #[derive(serde::Serialize)]
            struct GameWithCreator {
                game: game::Model,
                creator: crate::models::creator::Model,
            }

            HttpResponse::Ok().json(GameWithCreator { game, creator })
        }
        Ok(Some((_game, None))) => HttpResponse::NotFound().body("Creator not found"),
        Ok(None) => HttpResponse::NotFound().body("Game not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
