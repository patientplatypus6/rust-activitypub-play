extern crate dotenv;

use chrono::prelude::*;
use dotenv::dotenv;

use reqwest;

use rust_activitypub_play::http_signatures;
use rust_activitypub_play::{config, config::CONFIG};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

use activitystreams::{
    activity::ActorAndObject,
    actor::{Actor, ApActor, Person},
    context,
    iri_string::types::IriString,
    prelude::*,
    security,
    unparsed::UnparsedMutExt,
};
use activitystreams_ext::{Ext1, UnparsedExtension};

use serde_json;
use serde_json::json;

use http::{Request, Response};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    config::init();

    let date = Utc::now();
    let date_rfc822 = date.to_rfc2822().replace("+0000", "GMT");

    let object = json!({
            "id": format!("https://d012-71-36-108-249.ngrok.io/@doctor/{}", date_rfc822),
            "type": "Note",
            "published": date.to_rfc3339(),
            "attributedTo": "https://d012-71-36-108-249.ngrok.io/@doctor/actor.json",
            "inReplyTo": "https://dev.mastodon.lmorchard.com/@lmorchard/109339034898409760",
            "content": format!("<p>Hello at - {}</p>", date_rfc822),
            "to": "https://www.w3.org/ns/activitystreams#Public"
    });

    let document = json!({
        "@context": "https://www.w3.org/ns/activitystreams",

        "id": format!("https://d012-71-36-108-249.ngrok.io/@doctor/create-{}", date_rfc822),
        "type": "Create",
        "actor": "https://d012-71-36-108-249.ngrok.io/@doctor/actor.json",

        "object": object
    });

    let body = reqwest::get("http://localhost:8080/.well-known/webfinger?resource=ti")
        .await?
        .text()
        .await?;

    println!("body = {:?}", body);

    // let client = reqwest::Client::builder()
    //     .build()?;

    // Perform the actual execution of the network request
    // let res = client
    //     .get("localhost:8080/.well-known/webfinger?resource=something")
    //     .send()
    //     .await?;

    // Parse the response body as Json in this case
    // let ip = res
    //     .json::<HashMap<String, String>>()
    //     .await?;

    // println!("IP {:?}", ip);

    let document_string = serde_json::to_string_pretty(&document).unwrap();
    println!("DOC {}", document_string);



    Ok(())
}

