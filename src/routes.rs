use actix_web::web;
use crate::controllers::{creator_controller, game_controller, auth_controller};
use crate::middleware::auth::AuthMiddleware;

pub fn config(cfg: &mut web::ServiceConfig) {
    // Auth routes (no middleware required)
    cfg.service(
        web::scope("/api/auth")
            .route("/register", web::post().to(auth_controller::register))
            .route("/login", web::post().to(auth_controller::login))
    );

    // Protected routes with auth middleware
    cfg.service(
        web::scope("/api/auth")
            .wrap(AuthMiddleware::new())
            .route("/me", web::get().to(auth_controller::current_user))
    );

    // Creator routes with role-based auth
    cfg.service(
        web::scope("/api/creators")
            .wrap(AuthMiddleware::with_role("admin".to_string()))
            .route("", web::post().to(creator_controller::create_creator))
            .route("", web::get().to(creator_controller::get_all_creators))
            .route("/{id}", web::get().to(creator_controller::get_creator_by_id))
            .route("/{id}", web::put().to(creator_controller::update_creator))
            .route("/{id}", web::delete().to(creator_controller::delete_creator))
            .route("/{id}/games", web::get().to(creator_controller::get_games_by_creator)),
    );

    // Game routes with role-based auth
    cfg.service(
        web::scope("/api/games")
            .wrap(AuthMiddleware::with_role("creator".to_string()))
            .route("", web::post().to(game_controller::create_game))
            .route("", web::get().to(game_controller::list_games))
            .route("/{id}", web::get().to(game_controller::get_game))
            .route("/{id}", web::put().to(game_controller::update_game))
            .route("/{id}", web::delete().to(game_controller::delete_game))
            .route("/{id}/with-creator", web::get().to(game_controller::get_game_with_creator)),
    );
}

