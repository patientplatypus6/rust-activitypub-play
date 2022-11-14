use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

use actix_web::{get, web, HttpResponse, Responder};

use crate::app::AppState;
use crate::config;
use crate::constants::*;

#[get("/@{name}/actor.json")]
pub async fn actors_service(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let name = path.into_inner();
    match actor_lookup(&name) {
        Err(_err) => HttpResponse::NotFound().finish(),
        Ok(result) => HttpResponse::Ok().body(serde_json::to_string_pretty(&result).unwrap()),
    }
}

pub fn actor_lookup(name: &String) -> Result<LocalActorPerson, ResolverError> {
    Ok(LocalActorPerson::new(&name))
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct ActorPerson {
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    pub id: String,
    #[serde(rename = "type")]
    pub actor_type: String,
    pub preferred_username: String,
    pub inbox: String,
    pub public_key: ActorPublicKey,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ActorPublicKey {
    pub id: String,
    pub owner: String,
    pub public_key_pem: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LocalActorPerson {
    pub name: String,
}

impl LocalActorPerson {
    pub fn new(name: &String) -> Self {
        LocalActorPerson {
            name: name.to_string(),
        }
    }

    pub fn actor_url(&self) -> String {
        format!("{}/@{}/actor.json", config::CONFIG.base_url, &self.name)
    }

    pub fn inbox_url(&self) -> String {
        format!("{}/@{}/inbox/", config::CONFIG.base_url, &self.name)
    }

    pub fn public_key(&self) -> String {
        fs::read_to_string("./public.pem").expect("Should be able to read public key")
    }
}

impl ::serde::Serialize for LocalActorPerson {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let ext = ActorPerson {
            context: vec![
                CONTEXT_ACTIVITYSTREAMS.to_string(),
                CONTEXT_SECURITY.to_string(),
            ],
            id: self.actor_url(),
            actor_type: ACTOR_TYPE_PERSON.to_string(),
            preferred_username: self.name.clone(),
            inbox: self.inbox_url(),
            public_key: ActorPublicKey {
                id: format!("{}#main-key", &self.actor_url()),
                owner: self.actor_url(),
                public_key_pem: self.public_key(),
            },
        };
        Ok(ext.serialize(serializer)?)
    }
}
