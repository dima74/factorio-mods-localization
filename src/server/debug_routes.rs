use rocket::get;

use crate::server::trigger_update::{get_installation_id_and_mods, get_trigger_update_mutex};
use crate::webhooks;

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
    let (installation_id, mods) = match get_installation_id_and_mods(&repo, subpath).await {
        Ok(value) => value,
        Err(value) => return value,
    };
    webhooks::on_repository_added(&repo, mods, installation_id).await;
    "Ok."
}

/// Overwrites all english file on crowdin based on github
#[get("/importEnglish?<repo>&<subpath>&<secret>")]
pub async fn import_english(
    repo: String,
    subpath: Option<String>,
    secret: Option<String>,
) -> &'static str {
    if secret != Some(dotenv::var("WEBSERVER_SECRET").unwrap()) {
        return "Missing secret";
    }
    let _lock = get_trigger_update_mutex().await;
    let (installation_id, mods) = match get_installation_id_and_mods(&repo, subpath).await {
        Ok(value) => value,
        Err(value) => return value,
    };
    webhooks::import_english(&repo, mods, installation_id).await;
    "Ok."
}
