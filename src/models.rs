use std::fs;
use chrono::prelude::*;
use crate::app::AppState;
use crate::config;
use crate::constants::*;
use crate::objects::ObjectNote;
use serde::{Deserialize, Serialize};

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

    pub fn private_key(&self) -> String {
        fs::read_to_string("./private.pem").expect("Should be able to read public key")
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ActivityCreateNote {
    #[serde(rename = "@context")]
    pub context: String,
    pub id: String,
    #[serde(rename = "type")]
    pub activity_type: String,
    pub actor: String,
    pub object: ObjectNote,
}

impl ActivityCreateNote {
    pub fn new(actor: &LocalActorPerson, id: &String) -> Self {
        ActivityCreateNote {
            context: CONTEXT_ACTIVITYSTREAMS.to_string(),
            id: id.clone(),
            activity_type: ACTIVITY_TYPE_CREATE.to_string(),
            actor: actor.actor_url(),
            object: ObjectNote::new(
                &"bar".to_string(),
                &actor.actor_url(),
                &"https://dev.mastodon.lmorchard.com/@lmorchard/109339034898409760".to_string(),
                &"hello world".to_string(),
            ),
        }
    }
}