use std::path::Path;

use jsonwebtoken::EncodingKey;
use log::info;
use octocrab::{Octocrab, Page};
use octocrab::models::{AppId, Installation, InstallationId, Repository};
use rocket::serde::Deserialize;
use serde::de::DeserializeOwned;

use crate::{git_util, util};
use crate::mod_directory::ModDirectory;
use crate::util::EmptyBody;

fn get_credentials() -> (AppId, EncodingKey) {
    let github_app_id: u64 = dotenv::var("GITHUB_APP_ID").unwrap().parse().unwrap();
    let github_app_key = dotenv::var("GITHUB_APP_PRIVATE_KEY").unwrap().replace("\\n", "\n");
    let github_app_key = EncodingKey::from_rsa_pem(github_app_key.as_bytes()).unwrap();
    (AppId(github_app_id), github_app_key)
}

pub fn as_app() -> Octocrab {
    let (app_id, key) = get_credentials();
    Octocrab::builder().app(app_id, key).build().unwrap()
}

pub fn as_installation(installation_id: InstallationId) -> Octocrab {
    as_app().installation(installation_id)
}

// for tests/examples
pub async fn as_installation_for_user(login: &str) -> Octocrab {
    let api = as_app();
    let installation_id = get_all_installations(&api).await
        .iter().find(|it| it.account.login == login).unwrap()
        .id;
    api.installation(installation_id)
}

const MAX_PER_PAGE: u8 = 100;

pub async fn get_installation_id_for_user(login: &str) -> InstallationId {
    let api = as_app();
    get_all_installations(&api).await
        .iter().find(|it| it.account.login == login).unwrap()
        .id
}

pub async fn get_installation_id_for_repo(full_name: &str) -> Option<InstallationId> {
    let (owner, repo) = full_name.split_once('/').unwrap();
    as_app()
        .apps()
        .get_repository_installation(owner, repo).await
        .map(|it| it.id)
        .ok()
}

pub async fn has_repository_selection_all(installation_id: InstallationId) -> bool {
    let installation = as_app().apps().installation(installation_id).await.unwrap();
    let repository_selection = installation.repository_selection.unwrap();
    util::has_repository_selection_all(&repository_selection)
}

async fn has_locale_en(installation_api: &Octocrab, full_name: &str) -> bool {
    let (owner, repo) = full_name.split_once('/').unwrap();
    let response = installation_api
        .repos(owner, repo)
        .get_content()
        .path("locale/en")
        .send()
        .await;
    match response {
        Ok(response) => !response.items.is_empty(),
        Err(_) => false,
    }
}

pub async fn filter_repositories_with_locale_en(installation_api: &Octocrab, repositories: Vec<String>) -> Vec<String> {
    let mut result = Vec::new();
    for repository in repositories {
        if has_locale_en(installation_api, &repository).await {
            result.push(repository);
        }
    }
    result
}

trait PageExt<T> {
    async fn all_pages(self, api: &Octocrab) -> octocrab::Result<Vec<T>>;
}

impl<T: DeserializeOwned> PageExt<T> for Page<T> {
    async fn all_pages(self, api: &Octocrab) -> octocrab::Result<Vec<T>> {
        api.all_pages(self).await
    }
}

pub async fn get_all_installations(api: &Octocrab) -> Vec<Installation> {
    api
        .apps().installations().per_page(MAX_PER_PAGE)
        .send().await.unwrap()
        .all_pages(api).await.unwrap()
}

pub async fn get_all_repositories(api: &Octocrab) -> Vec<(String, InstallationId)> {
    let mut result = Vec::new();
    let installations = get_all_installations(api).await;
    for installation in installations {
        let installation_api = api.installation(installation.id);
        let repositories = get_repositories_of_installation(&installation_api).await;
        for repository in repositories {
            result.push((repository, installation.id));
        }
    }
    result
}

pub async fn get_repositories_of_installation(installation_api: &Octocrab) -> Vec<String> {
    let parameters = serde_json::json!({"per_page": MAX_PER_PAGE});
    let repositories: Page<Repository> = installation_api
        .get("/installation/repositories", Some(&parameters)).await.unwrap();
    let repositories = repositories.all_pages(&installation_api).await.unwrap();
    let repositories = repositories
        .into_iter()
        .filter(|it| !it.private.unwrap())
        .map(|it| it.full_name.unwrap())
        .collect();
    filter_repositories_with_locale_en(installation_api, repositories).await
}

pub async fn clone_repository(full_name: &str, installation_id: InstallationId) -> ModDirectory {
    info!("[{}] clone repository", full_name);
    use tempfile::TempDir;
    let directory = TempDir::with_prefix("FML.").unwrap();
    clone_repository_to(full_name, installation_id, directory.path()).await;
    ModDirectory::new(full_name, directory)
}

async fn clone_repository_to(full_name: &str, installation_id: InstallationId, path: &Path) {
    use secrecy::ExposeSecret;
    let api = as_app();
    let (_, installation_token) = api.installation_and_token(installation_id).await.unwrap();
    let installation_token = installation_token.expose_secret();
    let url = format!("https://x-access-token:{}@github.com/{}.git", installation_token, full_name);
    git_util::clone(&url, path);
}

async fn get_default_branch(installation_api: &Octocrab, full_name: &str) -> String {
    #[derive(Deserialize)]
    struct Response { default_branch: String }
    let url = format!("/repos/{}", full_name);
    let response: Response = installation_api.get(&url, None::<&()>).await.unwrap();
    response.default_branch
}

pub async fn is_default_branch_protected(installation_api: &Octocrab, full_name: &str) -> bool {
    let branch = get_default_branch(installation_api, full_name).await;
    is_branch_protected(installation_api, full_name, &branch).await
}

async fn is_branch_protected(installation_api: &Octocrab, full_name: &str, branch: &str) -> bool {
    #[derive(Deserialize)]
    struct Response { protected: bool }
    let url = format!("/repos/{}/branches/{}", full_name, branch);
    let result: Response = installation_api.get(&url, None::<&()>).await.unwrap();
    return result.protected
}

pub fn as_personal_account() -> Octocrab {
    let personal_token = dotenv::var("GITHUB_PERSONAL_ACCESS_TOKEN").unwrap();
    Octocrab::builder()
        .personal_token(personal_token)
        .build()
        .unwrap()
}

pub async fn star_repository(api: &Octocrab, full_name: &str) {
    let _response: octocrab::Result<EmptyBody> = api
        .put(format!("/user/starred/{}", full_name), None::<&()>)
        .await;
}

pub async fn is_repository_starred(api: &Octocrab, full_name: &str) -> bool {
    let response: octocrab::Result<EmptyBody> = api
        .get(format!("/user/starred/{}", full_name), None::<&()>)
        .await;
    response.is_ok()
}

pub async fn get_not_starred_repositories() -> Vec<String> {
    let api_app = as_app();
    let repositories = get_all_repositories(&api_app).await;

    let api_personal = as_personal_account();
    let mut not_starred = Vec::new();
    for (full_name, _id) in repositories {
        if !is_repository_starred(&api_personal, &full_name).await {
            not_starred.push(full_name);
        }
    }
    not_starred
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_has_locale_en() {
        let api = as_installation_for_user("dima74").await;
        assert!(has_locale_en(&api, "dima74/factorio-mod-example").await);
        assert!(!has_locale_en(&api, "dima74/factorio-mods-localization").await);
    }
}
