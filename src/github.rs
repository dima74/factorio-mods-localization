use std::ops::Deref;
use std::path::Path;
use std::time::Duration;

use jsonwebtoken::EncodingKey;
use log::info;
use octocrab::{Error, Octocrab, Page};
use octocrab::models::{AppId, Installation, InstallationId, Repository};
use octocrab::models::pulls::PullRequest;
use octocrab::models::repos::ContentItems;
use rocket::serde::Deserialize;
use serde::de::DeserializeOwned;
use tokio::time::sleep;

use crate::git_util;
use crate::github_config::parse_github_repo_info_json;
use crate::github_repo_info::{GithubRepoInfo};
use crate::mod_directory::RepositoryDirectory;
use crate::myenv::{GITHUB_APP_ID, GITHUB_APP_PRIVATE_KEY, GITHUB_PERSONAL_ACCESS_TOKEN};
use crate::sentry::sentry_report_error;
use crate::util::{create_temporary_directory, EmptyBody};

pub const GITHUB_USER_NAME: &str = "factorio-mods-helper";
pub const GITHUB_BRANCH_NAME: &str = "crowdin-fml";
pub const GITHUB_CONFIG_FILE_NAME: &str = "factorio-mods-localization.json";
const MAX_PER_PAGE: u8 = 100;

fn get_credentials() -> (AppId, EncodingKey) {
    let github_app_id: u64 = GITHUB_APP_ID.deref().parse().unwrap();
    let github_app_key = GITHUB_APP_PRIVATE_KEY.deref().replace("\\n", "\n");
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
pub async fn as_installation_for_user(login: &str) -> Option<Octocrab> {
    let api = as_app();
    let installation = get_all_installations(&api).await
        .into_iter()
        .find(|it| it.account.login == login)?;
    Some(api.installation(installation.id))
}

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

#[derive(Debug, Eq, PartialEq)]
pub enum GetRepoInfoError {
    InvalidConfig,
    LocaleDirectoryMissing,
    LocaleEnDirectoryMissingOrEmpty,
}

pub async fn get_repo_info(
    installation_api: &Octocrab,
    full_name: &str,
) -> Result<GithubRepoInfo, GetRepoInfoError> {
    let root_items = list_files_in_directory(installation_api, full_name, "").await.unwrap();
    if root_items.iter().any(|it| it == GITHUB_CONFIG_FILE_NAME) {
        let mods_file = get_content(installation_api, full_name, GITHUB_CONFIG_FILE_NAME).await.unwrap();
        let json = mods_file.items[0].decoded_content().unwrap();
        parse_github_repo_info_json(full_name, &json)
            .ok_or(GetRepoInfoError::InvalidConfig)
    } else {
        if !root_items.iter().any(|it| it == "locale") {
            return Err(GetRepoInfoError::LocaleDirectoryMissing);
        }
        let locale_en_items = list_files_in_directory(installation_api, full_name, "locale/en").await;
        match locale_en_items {
            Ok(locale_en_items) if !locale_en_items.is_empty() => {
                Ok(GithubRepoInfo::new_single_mod(full_name))
            }
            _ => {
                Err(GetRepoInfoError::LocaleEnDirectoryMissingOrEmpty)
            }
        }
    }
}

async fn get_content(installation_api: &Octocrab, full_name: &str, path: &str) -> octocrab::Result<ContentItems> {
    let (owner, repo) = full_name.split_once('/').unwrap();
    let result = installation_api
        .repos(owner, repo)
        .get_content()
        .path(path)
        .send()
        .await;
    if let Err(Error::GitHub { source, .. }) = &result {
        if path.is_empty() && source.errors.is_none() && source.message == "This repository is empty." {
            return Ok(ContentItems { items: vec![] });
        }
    }
    result
}

pub async fn list_files_in_directory(installation_api: &Octocrab, full_name: &str, path: &str) -> octocrab::Result<Vec<String>> {
    get_content(installation_api, full_name, path).await
        .map(|it| {
            it.items
                .into_iter()
                .map(|file| file.name)
                .collect()
        })
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

pub async fn get_all_repositories(api: &Octocrab) -> Vec<(GithubRepoInfo, InstallationId)> {
    let mut result = Vec::new();
    let installations = get_all_installations(api).await;
    for installation in installations {
        let installation_api = api.installation(installation.id);
        let repositories = get_all_repositories_of_installation(&installation_api).await;
        for repository in repositories {
            let repo_info = get_repo_info(&installation_api, &repository).await;
            if let Ok(repo_info) = repo_info {
                result.push((repo_info, installation.id));
            }
        }
    }
    result
}

pub async fn get_all_repositories_of_installation(installation_api: &Octocrab) -> Vec<String> {
    let parameters = serde_json::json!({"per_page": MAX_PER_PAGE});
    let repositories: Page<Repository> = installation_api
        .get("/installation/repositories", Some(&parameters)).await.unwrap();
    let repositories = repositories.all_pages(installation_api).await.unwrap();
    repositories
        .into_iter()
        .filter(|it| !it.private.unwrap())
        .map(|it| it.full_name.unwrap())
        .collect()
}

pub async fn clone_repository(
    repo_info: &GithubRepoInfo,
    installation_id: InstallationId,
) -> RepositoryDirectory {
    info!("[{}] clone repository", repo_info.full_name);
    let directory = create_temporary_directory();
    clone_repository_to(repo_info, installation_id, directory.path()).await;
    RepositoryDirectory::new(&repo_info.full_name, directory)
}

async fn clone_repository_to(
    repo_info: &GithubRepoInfo,
    installation_id: InstallationId,
    path: &Path,
) {
    use secrecy::ExposeSecret;
    let api = as_app();
    let (_, installation_token) = api.installation_and_token(installation_id).await.unwrap();
    let installation_token = installation_token.expose_secret();
    let url = format!("https://x-access-token:{}@github.com/{}.git", installation_token, repo_info.full_name);
    git_util::clone(&url, path, repo_info.branch.as_deref());
}

pub async fn create_pull_request(personal_api: &Octocrab, full_name: &str, base_branch: &str) {
    let (owner, repo) = full_name.split_once('/').unwrap();
    let title = "Update translations from Crowdin";
    let body = "See https://github.com/dima74/factorio-mods-localization for details";
    let head_branch = format!("{}:{}", GITHUB_USER_NAME, GITHUB_BRANCH_NAME);
    let result = personal_api
        .pulls(owner, repo)
        .create(title, head_branch, base_branch)
        .body(body)
        .maintainer_can_modify(true)
        .send().await;
    check_create_pull_request_response(result, full_name);
}

fn check_create_pull_request_response(result: octocrab::Result<PullRequest>, full_name: &str) {
    let Err(err) = result else { return; };
    if is_error_pull_request_already_exists(&err) {
        // PR exists - no need to reopen, force push is enough
        return;
    }
    if is_error_repository_archived(&err) {
        // Ignore archived repositories, can't create PRs for them
        return;
    }
    panic!("[{}] Can't create pull request: {}", full_name, err);
}

fn is_error_pull_request_already_exists(error: &Error) -> bool {
    let Error::GitHub { source, .. } = &error else { return false; };
    if source.message != "Validation Failed" { return false; };
    let Some([error, ..]) = source.errors.as_deref() else { return false; };
    let serde_json::Value::Object(error) = error else { return false; };
    let Some(serde_json::Value::String(message)) = error.get("message") else { return false; };
    message.starts_with("A pull request already exists for")
}

fn is_error_repository_archived(error: &Error) -> bool {
    let error_str = format!("{}", error);
    error_str.contains("Repository was archived so is read-only")
}

pub async fn get_default_branch(installation_api: &Octocrab, full_name: &str) -> String {
    #[derive(Deserialize)]
    struct Response { default_branch: String }
    let url = format!("/repos/{}", full_name);
    let response: Response = installation_api.get(&url, None::<&()>).await.unwrap();
    response.default_branch
}

pub async fn is_branch_protected(installation_api: &Octocrab, full_name: &str, branch: &str) -> bool {
    #[derive(Deserialize)]
    struct Response { protected: bool }
    let url = format!("/repos/{}/branches/{}", full_name, branch);
    let result: Response = installation_api.get(&url, None::<&()>).await.unwrap();
    result.protected
}

pub fn as_personal_account() -> Octocrab {
    let personal_token = GITHUB_PERSONAL_ACCESS_TOKEN.to_owned();
    Octocrab::builder()
        .personal_token(personal_token)
        .build()
        .unwrap()
}

pub async fn fork_repository(personal_api: &Octocrab, full_name: &str) -> bool {
    if let Some(is_fork_name_correct) = check_fork_exists(personal_api, full_name).await {
        return is_fork_name_correct;
    }
    fork_repository_without_check(personal_api, full_name).await;
    true
}

pub async fn fork_repository_without_check(personal_api: &Octocrab, full_name: &str) {
    let (owner, repo) = full_name.split_once('/').unwrap();
    info!("[{}] forking repository...", full_name);
    personal_api
        .repos(owner, repo)
        .create_fork()
        .send().await.unwrap();
    sleep(Duration::from_secs(120)).await;
}

// None => no fork
// Some(false) => fork with different name
// Some(true) => fork exists and can be used
async fn check_fork_exists(api: &Octocrab, full_name: &str) -> Option<bool> {
    let (owner, repo) = full_name.split_once('/').unwrap();
    let forks = api
        .repos(owner, repo)
        .list_forks()
        .send().await.unwrap()
        .all_pages(api).await.unwrap();
    for fork in forks {
        let fork_full_name = fork.full_name.unwrap();
        let (fork_owner, fork_repo) = fork_full_name.split_once('/').unwrap();
        if fork_owner == GITHUB_USER_NAME {
            return if fork_repo == repo {
                Some(true)  // fork already exists
            } else {
                let message = format!("Fork name {} doesn't match repository {}/{}", fork_repo, owner, repo);
                sentry_report_error(&message);
                Some(false)
            };
        }
    }
    None
}

#[derive(Default)]
pub struct GetNotForkedResult {
    pub not_forked: Vec<String>,
    pub forked_with_diferrent_name: Vec<String>,
}

pub async fn get_not_forked_repositories() -> GetNotForkedResult {
    let api_app = as_app();
    let repositories = get_all_repositories(&api_app).await;

    let api_personal = as_personal_account();
    let mut result = GetNotForkedResult::default();
    for (repo_info, _id) in repositories {
        let full_name = repo_info.full_name;
        match check_fork_exists(&api_personal, &full_name).await {
            None => result.not_forked.push(full_name),
            Some(false) => result.forked_with_diferrent_name.push(full_name),
            Some(true) => continue,
        }
    }
    result
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
    for (repo_info, _id) in repositories {
        let full_name = repo_info.full_name;
        if !is_repository_starred(&api_personal, &full_name).await {
            not_starred.push(full_name);
        }
    }
    not_starred
}

pub async fn get_current_user(api_oauth: &Octocrab) -> String {
    let response: octocrab::models::Author = api_oauth
        .get("/user", None::<&()>)
        .await.unwrap();
    response.login
}

#[cfg(test)]
mod tests {
    use crate::github_repo_info::GithubModInfo;

    use super::*;

    #[tokio::test]
    async fn test_has_locale_en() {
        let api = as_installation_for_user("dima74").await.unwrap();
        assert_eq!(
            get_repo_info(&api, "dima74/factorio-mod-example").await,
            Ok(GithubRepoInfo {
                full_name: "dima74/factorio-mod-example".to_owned(),
                mods: vec![GithubModInfo::new_root("dima74/factorio-mod-example")],
                weekly_update_from_crowdin: true,
                branch: None,
            }),
        );
        assert_eq!(
            get_repo_info(&api, "dima74/factorio-multimod-example").await,
            Ok(GithubRepoInfo {
                full_name: "dima74/factorio-multimod-example".to_owned(),
                mods: vec![
                    GithubModInfo {
                        owner: "dima74".to_owned(),
                        repo: "factorio-multimod-example".to_owned(),
                        locale_path: "Mod1/locale".to_owned(),
                        crowdin_name: Some("Name1".to_owned()),
                    },
                    GithubModInfo {
                        owner: "dima74".to_owned(),
                        repo: "factorio-multimod-example".to_owned(),
                        locale_path: "Mod3/Data/locale".to_owned(),
                        crowdin_name: Some("Name3".to_owned()),
                    },
                ],
                weekly_update_from_crowdin: true,
                branch: None,
            }),
        );
        assert_eq!(
            get_repo_info(&api, "dima74/factorio-mods-localization").await,
            Err(GetRepoInfoError::LocaleDirectoryMissing),
        );
    }
}
