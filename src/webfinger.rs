use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json;

use crate::actors::actor_lookup;
use crate::config;
use crate::constants::*;

#[get("/.well-known/webfinger")]
pub async fn resolver_service(info: web::Query<WebfingerParams>) -> impl Responder {
    match resolver(&info.resource) {
        Err(_err) => HttpResponse::NotFound().finish(),
        Ok(result) => HttpResponse::Ok().body(serde_json::to_string_pretty(&result).unwrap()),
    }
}

pub fn resolver(resource: &String) -> Result<WebfingerResult, ResolverError> {
    let mut parsed_query = resource.splitn(2, ':');
    let res_prefix = parsed_query.next().ok_or(ResolverError::InvalidResource)?;
    if res_prefix != "acct" {
        return Err(ResolverError::InvalidResource);
    }
    let res = parsed_query.next().ok_or(ResolverError::InvalidResource)?;
    let mut parsed_res = res.splitn(2, '@');
    let user = parsed_res.next().ok_or(ResolverError::InvalidResource)?;
    let domain = parsed_res.next().ok_or(ResolverError::InvalidResource)?;
    if domain != config::CONFIG.domain {
        // TODO: match multiple domains?
        return Err(ResolverError::WrongDomain);
    }

    match actor_lookup(&user.to_string()) {
        Err(_err) => Err(ResolverError::NotFound),
        Ok(actor) => Ok(WebfingerResult {
            subject: resource.clone(),
            links: vec![WebfingerLink {
                rel: WEBFINGER_ACTOR_REL.to_string(),
                mime_type: WEBFINGER_ACTOR_MEDIA_TYPE.to_string(),
                href: actor.actor_id(),
            }],
        }),
    }
}

/// Query parameters for webfinger resolver service
#[derive(Deserialize)]
pub struct WebfingerParams {
    resource: String,
}

/// WebFinger result that may serialized or deserialized to JSON
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct WebfingerResult {
    pub subject: String,
    pub links: Vec<WebfingerLink>,
}

/// Structure to represent a WebFinger link
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct WebfingerLink {
    pub rel: String,
    #[serde(rename = "type")]
    pub mime_type: String,
    pub href: String,
}

/// An error that occured while handling an incoming WebFinger request.
#[derive(Debug, PartialEq)]
pub enum ResolverError {
    /// The requested resource was not correctly formatted
    InvalidResource,
    /// The requested resource was not found.
    NotFound,
    /// The website of the resource is not the current one.
    WrongDomain,
}
