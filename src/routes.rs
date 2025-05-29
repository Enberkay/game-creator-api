use actix_web::web;
use crate::controllers::{creator_controller, game_controller};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/creators")
            .route("", web::post().to(creator_controller::create_creator))
            .route("", web::get().to(creator_controller::get_all_creators))
            .route("/{id}", web::get().to(creator_controller::get_creator_by_id))
            .route("/{id}", web::put().to(creator_controller::update_creator))
            .route("/{id}", web::delete().to(creator_controller::delete_creator))
            .route("/{id}/games", web::get().to(creator_controller::get_games_by_creator)),
    );

    cfg.service(
        web::scope("/api/games")
            .route("", web::post().to(game_controller::create_game))
            .route("", web::get().to(game_controller::list_games))
            .route("/{id}", web::get().to(game_controller::get_game))
            .route("/{id}", web::put().to(game_controller::update_game))
            .route("/{id}", web::delete().to(game_controller::delete_game))
            .route("/{id}/with-creator", web::get().to(game_controller::get_game_with_creator)),
    );
}
