extern crate dotenv;

use chrono::prelude::*;
use dotenv::dotenv;

use rust_activitypub_play::http_signatures;
use rust_activitypub_play::{config, config::CONFIG};
use sha2::{Digest, Sha256};

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

    let document_string = serde_json::to_string_pretty(&document).unwrap();
    println!("DOC {}", document_string);

    /*
    let mut hasher = Sha256::new();
    hasher.update(document.as_bytes());
    let result = hasher.finalize();
    let document_digest = format!("SHA-256={}", base64::encode(result));

    let to_sign = format!(
        "(request-target): post /inbox\nhost: dev.mastodon.lmorchard.com\ndate: {}\ndigest: {}",
        date_rfc822,
        document_digest,
    );
    let actor = LocalActorPerson::new(&"doctor".to_string());
    let private_key = http_signatures::parse_private_key(&actor.private_key()).unwrap();
    let signature = http_signatures::sign_string_with_private_key(private_key, &to_sign).unwrap();
    let signature_header = format!(
        r#"keyId="https://d012-71-36-108-249.ngrok.io/@doctor/actor.json#main-key",headers="(request-target) host date digest",signature="{}""#,
        signature,
    );

    let client = reqwest::Client::new();
    let resp = client
        .post("https://dev.mastodon.lmorchard.com/inbox")
        .header("Host", "dev.mastodon.lmorchard.com")
        .header("Date", date_rfc822)
        .header("Signature", signature_header)
        .header("Digest", document_digest)
        .body(document)
        .send()
        .await?
        .text()
        .await?;

    println!("{:#?}", resp);
    */
    Ok(())
}

/*
#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKeyInner {
    pub id: IriString,
    pub owner: IriString,
    pub public_key_pem: String,
}

impl std::fmt::Debug for PublicKeyInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PublicKeyInner")
            .field("id", &self.id.to_string())
            .field("owner", &self.owner.to_string())
            .field("public_key_pem", &self.public_key_pem)
            .finish()
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKey {
    pub public_key: PublicKeyInner,
}

impl<U> UnparsedExtension<U> for PublicKey
where
    U: UnparsedMutExt,
{
    type Error = serde_json::Error;

    fn try_from_unparsed(unparsed_mut: &mut U) -> Result<Self, Self::Error> {
        Ok(PublicKey {
            public_key: unparsed_mut.remove("publicKey")?,
        })
    }

    fn try_into_unparsed(self, unparsed_mut: &mut U) -> Result<(), Self::Error> {
        unparsed_mut.insert("publicKey", self.public_key)?;
        Ok(())
    }
}

pub type ExtendedPerson = Ext1<ApActor<Person>, PublicKey>;

fn main() -> Result<(), anyhow::Error> {
    let actor = ApActor::new("http://in.box".parse()?, Person::new());

    let mut person = Ext1::new(
        actor,
        PublicKey {
            public_key: PublicKeyInner {
                id: "http://key.id".parse()?,
                owner: "http://owner.id".parse()?,
                public_key_pem: "asdfasdfasdf".to_owned(),
            },
        },
    );

    person.set_context(context()).add_context(security());

    let any_base = person.into_any_base()?;
    //println!("any_base: {:#?}", any_base);
    let person = ExtendedPerson::from_any_base(any_base)?;

    println!("person: {}", serde_json::to_string_pretty(&person).unwrap());
    Ok(())
}
*/
