use chrono::prelude::*;
use rust_activitypub_play::http_signatures;
use rust_activitypub_play::models::LocalActorPerson;
use sha2::{Digest, Sha256};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let date = Utc::now();
    let date_rfc822 = date.to_rfc2822().replace("+0000", "GMT");
    let document = r#"{
        "@context": "https://www.w3.org/ns/activitystreams",

        "id": "https://d012-71-36-108-249.ngrok.io/@doctor/create-^^ID^^",
        "type": "Create",
        "actor": "https://d012-71-36-108-249.ngrok.io/@doctor/actor.json",

        "object": {
            "id": "https://d012-71-36-108-249.ngrok.io/@doctor/^^ID^^",
            "type": "Note",
            "published": "^^PUBLISHED^^",
            "attributedTo": "https://d012-71-36-108-249.ngrok.io/@doctor/actor.json",
            "inReplyTo": "https://dev.mastodon.lmorchard.com/@lmorchard/109339034898409760",
            "content": "<p>Hello at - ^^MESSAGE^^</p>",
            "to": "https://www.w3.org/ns/activitystreams#Public"
        }
    }"#
    .to_string()
    .replace("^^ID^^", &date_rfc822)
    .replace("^^PUBLISHED^^", &date.to_rfc3339())
    .replace("^^MESSAGE^^", &date_rfc822);

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
    Ok(())
}
