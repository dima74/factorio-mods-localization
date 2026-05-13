//! `/api/triggerUpdateWithSecret?repo=OWNER/REPO&secret=...` - for public use, secret based authorization
//! Secret is per-user.

use std::collections::HashMap;
use std::ops::Deref;

use rocket::get;

use crate::myenv::WEBSERVER_SECRET_PUBLIC;
use crate::server::trigger_update::trigger_update_single_repository_part1;

#[get("/api/triggerUpdateWithSecret?<repo>&<secret>")]
pub async fn trigger_update(repo: Option<String>, secret: Option<String>) -> String {
    let Some(repo) = repo else {
        return "Missing `repo` query parameter".to_owned();
    };
    if repo.chars().filter(|it| *it == '/').count() != 1
        || repo.starts_with('/')
        || repo.ends_with('/')
    {
        return "`repo` parameter should be in format `owner/repo`".to_owned();
    }
    let Some(secret) = secret else {
        return "Missing `secret` query parameter".to_owned();
    };

    let owner = repo.split_once('/').unwrap().0;
    let secret_map: HashMap<String, String> = serde_json::from_str::<HashMap<String, String>>(WEBSERVER_SECRET_PUBLIC.deref()).unwrap();
    if !secret_map.contains_key(owner) {
        return "No secret configured for this repository owner. Please contact maintainer in private message.".to_owned();
    }
    if secret_map.get(owner) != Some(&secret) {
        return "Invalid secret".to_owned();
    }

    match trigger_update_single_repository_part1(repo, None).await {
        Ok(_) => "Triggered. Repository will be updated if there are changes on Crowdin.".to_owned(),
        Err(e) => e.to_owned(),
    }
}
