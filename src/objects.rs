use actix_web::{get, web, HttpResponse, Responder};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::app::AppState;
// use crate::config;
use crate::constants::*;

#[derive(Deserialize)]
pub struct NotesServicePathInfo {
    actor_name: String,
    note_id: String,
}

#[get("/@{actor_name}/notes/{note_id}.json")]
pub async fn notes_service(
    path: web::Path<NotesServicePathInfo>,
    data: web::Data<AppState>,
) -> impl Responder {
    match note_lookup(&path.actor_name, &path.note_id) {
        Err(_err) => HttpResponse::NotFound().finish(),
        Ok(result) => HttpResponse::Ok().body(serde_json::to_string_pretty(&result).unwrap()),
    }
}

pub fn note_lookup(actor_name: &String, note_id: &String) -> Result<ObjectNote, LookupError> {
    Ok(ObjectNote::new(
        &"bar".to_string(),
        &"bar".to_string(),
        &"bar".to_string(),
        &"bar".to_string(),
    ))
}

#[derive(Debug, PartialEq)]
pub enum LookupError {
    // NotFound,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ObjectNote {
    pub id: String,
    #[serde(rename = "type")]
    pub object_type: String,
    pub published: DateTime<Utc>,
    pub attributed_to: String,
    pub in_reply_to: String,
    pub content: String,
    pub to: String,
}

impl ObjectNote {
    pub fn new(
        id: &String,
        attributed_to: &String,
        in_reply_to: &String,
        content: &String,
    ) -> Self {
        ObjectNote {
            id: id.clone(),
            object_type: OBJECT_TYPE_NOTE.to_string(),
            published: Utc::now(),
            attributed_to: attributed_to.clone(),
            in_reply_to: in_reply_to.clone(),
            content: content.clone(),
            to: TO_PUBLIC.to_string(),
        }
    }
}
