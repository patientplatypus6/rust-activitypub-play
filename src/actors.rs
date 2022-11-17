use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::{json, Value};
use std::fs;

use actix_web::{get, web, HttpResponse, Responder};

use crate::config;
use crate::constants::*;

#[get("/@{name}/actor.json")]
pub async fn actors_service(path: web::Path<String>) -> impl Responder {
    let name = path.into_inner();
    match actor_lookup(&name) {
        Err(_err) => HttpResponse::NotFound().finish(),
        Ok(result) => match serde_json::to_string_pretty(&result.to_json_value()) {
            Err(_err) => HttpResponse::NotFound().finish(),
            Ok(body) => HttpResponse::Ok().body(body),
        },
    }
}

pub fn actor_lookup(name: &String) -> Result<LocalActorPerson, ResolverError> {
    Ok(LocalActorPerson::new(&name))
}

/// An error that occured while handling an incoming WebFinger request.
#[derive(Debug, PartialEq)]
pub enum ResolverError {
    /// The requested resource was not found.
    NotFound,
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

    pub fn actor_base_url(&self) -> String {
        format!("{}/@{}", config::CONFIG.base_url, &self.name)
    }

    pub fn actor_html_url(&self) -> String {
        format!("{}", self.actor_base_url())
    }

    pub fn actor_id(&self) -> String {
        format!("{}/actor.json", self.actor_base_url())
    }

    pub fn shared_inbox_url(&self) -> String {
        format!("{}/inbox", config::CONFIG.base_url)
    }

    pub fn inbox_url(&self) -> String {
        format!("{}/inbox", self.actor_base_url())
    }

    pub fn outbox_url(&self) -> String {
        format!("{}/outbox", self.actor_base_url())
    }

    pub fn public_key(&self) -> String {
        fs::read_to_string("./public.pem").expect("Should be able to read public key")
    }

    pub fn private_key(&self) -> String {
        fs::read_to_string("./private.pem").expect("Should be able to read public key")
    }

    pub fn to_json_value(&self) -> Value {
        json!({
            "@context": [
                CONTEXT_ACTIVITYSTREAMS.to_string(),
                CONTEXT_SECURITY.to_string(),
            ],
            "id": self.actor_id(),
            "type": ACTOR_TYPE_APPLICATION,
            "preferredUsername": self.name,
            "name": self.name,
            "url": self.actor_html_url(),
            "inbox": self.inbox_url(),
            "outbox": self.outbox_url(),
            /*
            "icon": {
                "type": "Image",
                "mediaType": "image/png",
                "url": "https://hackers.town/system/accounts/avatars/000/136/533/original/1a8c651efe14fcd6.png"
            },            
            */
            "endpoints": {
                "sharedInbox": self.shared_inbox_url(),
            },
            "publicKey": {
                "id": format!("{}#main-key", &self.actor_id()),
                "owner": self.actor_id(),
                "publicKeyPem": self.public_key(),
            }
        })
    }
}
