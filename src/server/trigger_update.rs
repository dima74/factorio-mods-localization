use std::fs;
use std::path::Path;
use std::sync::LazyLock;

use log::{info, warn};
use octocrab::models::InstallationId;
use rocket::get;
use tempfile::TempDir;

use crate::{crowdin, git_util, github, util};
use crate::crowdin::{get_crowdin_directory_name, normalize_language_code, replace_ini_to_cfg};
use crate::mod_directory::ModDirectory;

#[get("/triggerUpdate?<repo>&<secret>")]
pub async fn trigger_update(repo: Option<String>, secret: Option<String>) -> &'static str {
    if secret != Some(dotenv::var("WEBSERVER_SECRET").unwrap()) {
        return "Missing secret";
    }
    match repo {
        Some(repo) => {
            push_repository_crowdin_changes_to_github(&repo).await
        }
        None => {
            let task = push_all_crowdin_changes_to_github();
            tokio::spawn(task);
            // TODO link to logs
            "Triggered. See logs for details."
        }
    }
}

async fn get_trigger_update_mutex() -> impl Drop {
    // Note that tokio Mutex doesn't poisoning in contrast to stdlib Mutex.
    // This means that it will work correctly if thread panicked.
    use tokio::sync::Mutex;
    static MUTEX: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));
    MUTEX.lock().await
}

async fn push_repository_crowdin_changes_to_github(full_name: &str) -> &'static str {
    let _lock = get_trigger_update_mutex().await;
    info!("\n[update-github-from-crowdin] [{}] starting...", full_name);
    let Some(installation_id) = github::get_installation_id_for_repo(full_name).await else {
        return "Can't find github installation";
    };
    let repositories = vec![(full_name.to_owned(), installation_id)];
    let success = push_crowdin_changes_to_github(repositories).await;
    if !success {
        return "Can't find mod directory on crowdin"
    }
    info!("[update-github-from-crowdin] [{}] success", full_name);
    "Ok"
}

async fn push_all_crowdin_changes_to_github() {
    let _lock = get_trigger_update_mutex().await;
    info!("\n[update-github-from-crowdin] [*] starting...");
    let mut api = github::as_app();
    let repositories = github::get_all_repositories(&mut api).await;
    push_crowdin_changes_to_github(repositories).await;
    info!("[update-github-from-crowdin] [*] success");
}

async fn push_crowdin_changes_to_github(repositories: Vec<(String, InstallationId)>) -> bool {
    let repositories = crowdin::filter_repositories(repositories).await;
    if repositories.is_empty() { return false; }
    let translations_directory = crowdin::download_all_translations().await;
    for (repository, installation_id) in repositories {
        push_repository_crowdin_changes_to_github_impl(repository, installation_id, &translations_directory).await;
    }
    true
}

async fn push_repository_crowdin_changes_to_github_impl(
    full_name: String,
    installation_id: InstallationId,
    translations_directory: &TempDir,
) {
    let mod_directory = github::clone_repository(&full_name, installation_id).await;
    move_translated_files_to_repository(&mod_directory, translations_directory.path()).await;
    let path = mod_directory.root();
    let are_changes_exists = git_util::add_all_and_check_has_changes(path);
    if are_changes_exists {
        info!("[update-github-from-crowdin] [{}] found changes", full_name);
        let installation_api = github::as_installation(installation_id);
        let is_protected = github::is_default_branch_protected(&installation_api, &full_name).await;
        if is_protected {
            warn!("[update-github-from-crowdin] [{}] can't push because branch is protected", full_name);
        } else {
            git_util::commit_and_push(path);
            info!("[update-github-from-crowdin] [{}] pushed", full_name);
        }
    } else {
        info!("[update-github-from-crowdin] [{}] no changes found", full_name);
    }
}

async fn move_translated_files_to_repository(mod_directory: &ModDirectory, translation_directory: &Path) {
    for (language_path, language) in util::read_dir(translation_directory) {
        let language_path_crowdin = language_path.join(get_crowdin_directory_name(&mod_directory.github_full_name));
        assert!(language_path_crowdin.exists());
        let files = util::read_dir(&language_path_crowdin).collect::<Vec<_>>();
        if files.is_empty() { continue; }

        let language_original = util::read_dir(&mod_directory.locale_path())
            .map(|(_path, name)| name)
            .find(|it| normalize_language_code(it) == language)
            .unwrap_or(language);
        let language_path_repository = mod_directory.locale_path().join(language_original);
        fs::create_dir(&language_path_repository).ok();
        for (old_path, name) in files {
            assert!(name.ends_with(".ini"), "file {} from crowdin must ends with .ini`", name);
            let file_renamed = replace_ini_to_cfg(&name);
            let new_path = language_path_repository.join(&file_renamed);
            fs::rename(old_path, new_path).unwrap();
        }
    }
}
