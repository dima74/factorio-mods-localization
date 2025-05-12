use rocket::get;
use serde::Serialize;

use crate::{github, webhooks};
use crate::server::check_secret;
use crate::server::trigger_update::{get_installation_id_and_repo_info, get_trigger_update_mutex};

#[get("/listRepos?<secret>")]
pub async fn list_repositories(secret: Option<String>) -> String {
    if !check_secret(secret) { return "Missing secret".to_owned(); }
    let api = github::as_app();
    let repositories = github::get_all_repositories(&api).await
        .into_iter()
        .map(|(repo_info, _)| repo_info.full_name)
        .collect::<Vec<_>>();
    serde_json::to_string_pretty(&repositories).unwrap()
}

#[get("/listReposForUser?<user>&<secret>")]
pub async fn list_repositories_for_user(user: String, secret: Option<String>) -> String {
    if !check_secret(secret) { return "Missing secret".to_owned(); }
    let Some(api) = github::as_installation_for_user(&user).await else {
        return format!("App is not installed for {}", user);
    };
    let repositories = github::get_all_repositories_of_installation(&api).await;
    serde_json::to_string_pretty(&repositories).unwrap()
}

#[get("/listUsers?<secret>")]
pub async fn list_users(secret: Option<String>) -> String {
    #[derive(Serialize)]
    struct User {
        login: String,
        repository_selection: Option<String>,
    }

    if !check_secret(secret) { return "Missing secret".to_owned(); }
    let api = github::as_app();
    let repositories = github::get_all_installations(&api).await
        .into_iter()
        .map(|installation| User {
            login: installation.account.login,
            repository_selection: installation.repository_selection,
        })
        .collect::<Vec<_>>();
    serde_json::to_string_pretty(&repositories).unwrap()
}

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
    webhooks::import_english(repo_info, installation_id).await;
    "Ok."
}

#[get("/triggerOOM?<secret>")]
pub async fn trigger_oom(secret: Option<String>) -> &'static str {
    if !check_secret(secret) { return "Missing secret"; }
    let _lock = get_trigger_update_mutex().await;

    eprintln!("\nTrying to trigger OOM...");
    let mut v = Vec::new();
    loop {
        v.push(v.len());
    }
}
