use rocket::get;

use crate::server::check_secret;
use crate::server::trigger_update::{get_installation_id_and_repo_info, get_trigger_update_mutex};
use crate::webhooks;

/// For cases when repository was not imported correctly for some reason and manual intervention is needed
#[get("/importRepository?<repo>&<subpath>&<secret>")]
pub async fn import_repository(
    repo: String,
    subpath: Option<String>,
    secret: Option<String>,
) -> &'static str {
    if !check_secret(secret) { return "Missing secret"; }
    let _lock = get_trigger_update_mutex().await;
    let (installation_id, repo_info) = match get_installation_id_and_repo_info(&repo, subpath).await {
        Ok(value) => value,
        Err(value) => return value,
    };
    webhooks::on_repository_added(repo_info, installation_id).await;
    "Ok."
}

/// Overwrites all english file on crowdin based on github
#[get("/importEnglish?<repo>&<subpath>&<secret>")]
pub async fn import_english(
    repo: String,
    subpath: Option<String>,
    secret: Option<String>,
) -> &'static str {
    if !check_secret(secret) { return "Missing secret"; }
    let _lock = get_trigger_update_mutex().await;
    let (installation_id, repo_info) = match get_installation_id_and_repo_info(&repo, subpath).await {
        Ok(value) => value,
        Err(value) => return value,
    };
    webhooks::import_english(&repo, repo_info, installation_id).await;
    "Ok."
}
