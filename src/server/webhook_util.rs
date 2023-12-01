use std::error::Error;

use hmac::{Hmac, Mac};
use octocrab::models::webhook_events::WebhookEvent;
use rocket::{Data, Request};
use rocket::data::{FromData, Outcome, ToByteUnit};
use rocket::http::Status;
use sha2::Sha256;

pub struct GithubEvent(pub WebhookEvent);

#[rocket::async_trait]
impl<'r> FromData<'r> for GithubEvent {
    type Error = String;

    async fn from_data(request: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
        match GithubEvent::from_data_impl(request, data).await {
            Ok(result) => Outcome::Success(result),
            Err(err) => {
                let message = format!("{}", err);
                Outcome::Failure((Status::BadRequest, message))
            }
        }
    }
}

impl GithubEvent {
    // https://docs.github.com/en/webhooks/webhook-events-and-payloads#delivery-headers
    async fn from_data_impl<'r>(request: &Request<'_>, data: Data<'r>) -> Result<Self, Box<dyn Error>> {
        // Parse the event type
        let event_type = request.headers()
            .get_one("X-Github-Event")
            .ok_or("Missing X-Github-Event header")?;

        // Parse the signature
        let signature = request
            .headers()
            .get_one("X-Hub-Signature-256")
            .and_then(|header| parse_signature(header))
            .ok_or("Invalid signature")?;

        // Read the data into a String
        let limit = request.limits().get("json").unwrap_or(1.mebibytes());
        let mut content = Vec::new();
        data.open(limit).stream_to(&mut content).await?;

        // Validate signature
        verify_signature(&signature, &content)?;

        let event = WebhookEvent::try_from_header_and_body(event_type, &content)?;
        Ok(GithubEvent(event))
    }
}

fn verify_signature(signature: &[u8], content: &[u8]) -> Result<(), impl Error> {
    let secret = dotenv::var("GITHUB_APP_WEBHOOKS_SECRET").unwrap();
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes())
        .expect("HMAC can take key of any size");
    mac.update(&content);
    mac.verify_slice(signature)
}

fn parse_signature(header: &str) -> Option<Vec<u8>> {
    let header = header.trim();
    let Some(digest) = header.strip_prefix("sha256=") else { return None; };
    hex::decode(digest).ok()
}
