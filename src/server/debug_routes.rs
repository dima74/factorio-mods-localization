use rocket::get;

use crate::{github, webhooks};
use crate::github::extract_mods_from_repository;
use crate::github_mod_name::GithubModName;
use crate::server::trigger_update::get_trigger_update_mutex;

/// For cases when repository was not imported correctly for some reason and manual intervention is needed
#[get("/importRepository?<repo>&<subpath>&<secret>")]
pub async fn import_repository(
    repo: String,
    subpath: Option<String>,
    secret: Option<String>,
) -> &'static str {
    if secret != Some(dotenv::var("WEBSERVER_SECRET").unwrap()) {
        return "Missing secret";
    }

    let _lock = get_trigger_update_mutex().await;
    let installation_id = match github::get_installation_id_for_repo(&repo).await {
        Some(id) => id,
        None => return "Can't find installation for repository",
    };

    let mods = if subpath.is_some() {
        vec![GithubModName::new(&repo, subpath)]
    } else {
        let api = github::as_installation(installation_id);
        extract_mods_from_repository(&api, &repo).await
    };
    if mods.is_empty() {
        return "No mods.";
    }
    webhooks::on_repository_added(&repo, mods, installation_id).await;
    "Ok."
}
