use rocket::get;

use crate::{github, webhooks};

/// For cases when repository was not imported correctly for some reason and manual intervention is needed
#[get("/importRepository?<repo>&<secret>")]
pub async fn import_repository(repo: String, secret: Option<String>) -> &'static str {
    if secret != Some(dotenv::var("WEBSERVER_SECRET").unwrap()) {
        return "Missing secret";
    }

    let installation_id = match github::get_installation_id_for_repo(&repo).await {
        Some(id) => id,
        None => return "Can't find installation for repository",
    };
    webhooks::on_repository_added(&repo, installation_id).await;
    "Ok."
}
