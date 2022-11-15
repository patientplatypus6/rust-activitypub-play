use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

use actix_web::{get, web, HttpResponse, Responder};

use crate::app::AppState;
use crate::config;
use crate::models::LocalActorPerson;
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
