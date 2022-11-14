#[macro_use]
extern crate lazy_static;

use actix_web::{web, App, HttpServer};

mod activities;
mod actors;
mod app;
mod config;
mod constants;
mod objects;
mod webfinger;
mod http_signatures;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(app::AppState::new()))
            .service(webfinger::resolver_service)
            .service(actors::actors_service)
            .service(objects::notes_service)
            .service(activities::activities_service)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
