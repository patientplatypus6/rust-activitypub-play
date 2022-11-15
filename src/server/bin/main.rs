use actix_web::{web, App, HttpServer};

use rust_activitypub_play::*;
use rust_activitypub_play::config::CONFIG;

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
    .bind((CONFIG.host.clone(), CONFIG.port))?
    .run()
    .await
}
