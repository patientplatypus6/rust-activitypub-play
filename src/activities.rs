use actix_web::{get, web, HttpResponse, Responder};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::actors;
use crate::app::AppState;
use crate::config;
use crate::models::LocalActorPerson;
use crate::constants::*;
use crate::objects::ObjectNote;

#[derive(Deserialize)]
pub struct ActivityCreateNoteServicePathInfo {
    actor_name: String,
    activity_id: String,
}

#[get("/@{actor_name}/activities/{activity_id}.json")]
pub async fn activities_service(
    path: web::Path<ActivityCreateNoteServicePathInfo>,
) -> impl Responder {
    let actor = actors::actor_lookup(&path.actor_name);
    if let Err(_) = actor {
        return HttpResponse::NotFound().finish();
    }
    let actor = actor.unwrap();

    match activity_lookup(&actor, &path.activity_id) {
        Err(_err) => HttpResponse::NotFound().finish(),
        Ok(result) => HttpResponse::Ok().body(serde_json::to_string_pretty(&result).unwrap()),
    }
}

pub fn activity_lookup(
    actor: &LocalActorPerson,
    activity_id: &String,
) -> Result<ActivityCreateNote, LookupError> {
    Ok(ActivityCreateNote::new(&actor, &activity_id))
}

#[derive(Debug, PartialEq)]
pub enum LookupError {
    NotFound,
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
