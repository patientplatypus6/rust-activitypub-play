use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;
use std::fs;

#[derive(Clone)]
struct AppState {
    app_name: String,
    public_key: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app_state = AppState {
            app_name: String::from("ActivityPub Play"),
            public_key: fs::read_to_string("./public.pem")
                .expect("Should be able to read public key"),
        };
        App::new()
            .app_data(web::Data::new(app_state))
            .service(hello)
            .service(echo)
            .service(webfinger)
            .service(actor)
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

#[get("/.well-known/webfinger")]
async fn webfinger(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body(String::from(&data.app_name))
}

#[get("/actor.json")]
async fn actor(data: web::Data<AppState>) -> impl Responder {
    let actor_json = json!({
        "@context": [
            "https://www.w3.org/ns/activitystreams",
            "https://w3id.org/security/v1"
        ],

        "id": "https://my-example.com/actor",
        "type": "Person",
        "preferredUsername": "alice",
        "inbox": "https://my-example.com/inbox",

        "publicKey": {
            "id": "https://my-example.com/actor#main-key",
            "owner": "https://my-example.com/actor",
            "publicKeyPem": data.public_key
        }
    });
    HttpResponse::Ok().body(actor_json.to_string())
}
