extern crate dotenv;

use log::{info};

use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_files::Files;

use rust_activitypub_play::*;
use rust_activitypub_play::{config, config::CONFIG};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    config::init();

    info!(
        "Server starting - host: {}; port: {}; domain: {}",
        CONFIG.host,
        CONFIG.port,
        CONFIG.domain
    );

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(app::AppState::new()))
            .service(webfinger::resolver_service)
            .service(actors::actors_service)
            .service(objects::notes_service)
            .service(activities::activities_service)
            .service(Files::new("/", "./static/").index_file("index.html"))
            .wrap(Logger::default())
    })
    .bind((CONFIG.host.clone(), CONFIG.port))?
    .run()
    .await
}
