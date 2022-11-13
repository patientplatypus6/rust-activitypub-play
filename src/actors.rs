use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

use actix_web::{get, web, HttpResponse, Responder};

use crate::app::AppState;
use crate::config;

static CONTEXT_ACTIVITYSTREAMS: &str = "https://www.w3.org/ns/activitystreams";
static CONTEXT_SECURITY: &str = "https://w3id.org/security/v1";
static ACTOR_TYPE_PERSON: &str = "Person";

#[get("/@{name}/actor.json")]
pub async fn actors_service(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let name = path.into_inner();
    match actor_lookup(&name) {
        Err(_err) => HttpResponse::NotFound().finish(),
        Ok(result) => HttpResponse::Ok().body(serde_json::to_string_pretty(&result).unwrap()),
    }
}

pub fn actor_url(name: &str) -> String {
    format!("{}/@{}/actor.json", config::CONFIG.base_url, name)
}

pub fn actor_inbox_url(name: &str) -> String {
    format!("{}/@{}/inbox/", config::CONFIG.base_url, name)
}

pub fn actor_public_key(_name: &str) -> String {
    fs::read_to_string("./public.pem").expect("Should be able to read public key")
}

pub fn actor_lookup(name: &String) -> Result<ActorPerson, ResolverError> {
    Ok(ActorPerson::new(&name))
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ActorPerson {
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    pub id: String,
    #[serde(rename = "type")]
    pub actor_type: String,
    pub preferred_username: String,
    pub inbox: String,
    pub public_key: ActorPublicKey,
}

impl ActorPerson {
    pub fn new(name: &String) -> Self {
        ActorPerson {
            context: vec![
                CONTEXT_ACTIVITYSTREAMS.to_string(),
                CONTEXT_SECURITY.to_string(),
            ],
            id: actor_url(&name),
            actor_type: ACTOR_TYPE_PERSON.to_string(),
            preferred_username: name.clone(),
            inbox: actor_inbox_url(&name),
            public_key: ActorPublicKey {
                id: format!("{}#main-key", actor_url(&name)),
                owner: actor_url(&name),
                public_key_pem: actor_public_key(&name),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ActorPublicKey {
    pub id: String,
    pub owner: String,
    pub public_key_pem: String,
}

/// An error that occured while handling an incoming WebFinger request.
#[derive(Debug, PartialEq)]
pub enum ResolverError {
    /// The requested resource was not correctly formatted
    InvalidResource,
    /*
    /// The website of the resource is not the current one.
    WrongDomain,
    /// The requested resource was not found.
    NotFound,
    */
}
