use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;

mod database;
mod routes;
mod controllers;
mod models;
mod dtos;
mod middleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db = database::connect().await.expect("Failed to connect to database");

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    println!("🚀 Server running at http://localhost:{port}");

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(db.clone()))
            .configure(routes::config)
    })
    .bind(("127.0.0.1", port.parse().unwrap()))?
    .run()
    .await
}
