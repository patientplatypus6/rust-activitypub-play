#[macro_use]
extern crate lazy_static;

use std::fs;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

mod app;
mod config;
mod actors;
mod webfinger_resolver;

use app::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app_state = AppState {
            public_key: fs::read_to_string("./public.pem")
                .expect("Should be able to read public key"),
        };
        App::new()
            .app_data(web::Data::new(app_state))
            .service(hello)
            .service(echo)
            .service(webfinger_resolver::resolver_service)
            .service(actors::actors_service)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
