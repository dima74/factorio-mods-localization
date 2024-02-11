use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::sync::LazyLock;
use std::time::Duration;

use log::info;
use octocrab::models::InstallationId;
use rocket::get;
use tempfile::TempDir;
use tokio::time::sleep;

use crate::{crowdin, git_util, github, util};
use crate::crowdin::{get_crowdin_directory_name, normalize_language_code, replace_ini_to_cfg};
use crate::github::{as_personal_account, extract_mods_from_repository};
use crate::github_mod_name::GithubModName;
use crate::mod_directory::ModDirectory;

#[get("/triggerUpdate?<repo>&<subpath>&<secret>")]
pub async fn trigger_update(
    repo: Option<String>,
    subpath: Option<String>,
    secret: Option<String>,
) -> &'static str {
    if secret != Some(dotenv::var("WEBSERVER_SECRET").unwrap()) {
        return "Missing secret";
    }
    match repo {
        Some(repo) => {
            trigger_update_single_repository(&repo, subpath).await
        }
        None => {
            let task = trigger_update_all_repositories();
            tokio::spawn(task);
            // TODO link to logs
            "Triggered. See logs for details."
        }
    }
}

pub async fn get_trigger_update_mutex() -> impl Drop {
    // Note that tokio Mutex doesn't poisoning in contrast to stdlib Mutex.
    // This means that it will work correctly if thread panicked.
    use tokio::sync::Mutex;
    static MUTEX: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));
    MUTEX.lock().await
}

async fn trigger_update_single_repository(full_name: &str, subpath: Option<String>) -> &'static str {
    let _lock = get_trigger_update_mutex().await;
    info!("\n[update-github-from-crowdin] [{}] starting...", full_name);
    let (installation_id, mods) = match get_installation_id_and_mods(&full_name, subpath).await {
        Ok(value) => value,
        Err(message) => return message,
    };
    let repositories = vec![(full_name.to_owned(), mods, installation_id)];
    let success = push_crowdin_changes_to_repositories(repositories).await;
    if !success {
        return "Can't find mod directory on crowdin";
    }
    info!("[update-github-from-crowdin] [{}] success", full_name);
    "Ok"
}

pub async fn get_installation_id_and_mods(
    repo: &str,
    subpath: Option<String>,
) -> Result<(InstallationId, Vec<GithubModName>), &'static str> {
    let installation_id = match github::get_installation_id_for_repo(&repo).await {
        Some(id) => id,
        None => return Err("Can't find installation for repository"),
    };

    let mods = if subpath.is_some() {
        vec![GithubModName::new(&repo, subpath)]
    } else {
        let api = github::as_installation(installation_id);
        extract_mods_from_repository(&api, &repo).await
    };
    if mods.is_empty() {
        return Err("No mods.");
    }
    Ok((installation_id, mods))
}

async fn trigger_update_all_repositories() {
    let _lock = get_trigger_update_mutex().await;
    info!("\n[update-github-from-crowdin] [*] starting...");
    let mut api = github::as_app();
    let repositories = github::get_all_repositories(&mut api).await;
    push_crowdin_changes_to_repositories(repositories).await;
    info!("[update-github-from-crowdin] [*] success");
}

async fn push_crowdin_changes_to_repositories(repositories: Vec<(String, Vec<GithubModName>, InstallationId)>) -> bool {
    let repositories = crowdin::filter_repositories(repositories).await;
    if repositories.is_empty() { return false; }
    let translations_directory = crowdin::download_all_translations().await;
    for (repository, mods, installation_id) in repositories {
        push_crowdin_changes_to_repository(repository, mods, installation_id, &translations_directory).await;
    }
    true
}

async fn push_crowdin_changes_to_repository(
    full_name: String,
    mods: Vec<GithubModName>,
    installation_id: InstallationId,
    translations_directory: &TempDir,
) {
    let repository_directory = github::clone_repository(&full_name, installation_id).await;
    for mod_ in mods {
        let mod_directory = ModDirectory::new(&repository_directory, mod_);
        move_translated_files_to_mod_directory(&mod_directory, translations_directory.path()).await;
    }
    let path = repository_directory.root.path();
    let are_changes_exists = git_util::add_all_and_check_has_changes(path);
    if are_changes_exists {
        info!("[update-github-from-crowdin] [{}] found changes", full_name);
        git_util::commit(path);
        let installation_api = github::as_installation(installation_id);
        let default_branch = github::get_default_branch(&installation_api, &full_name).await;
        let is_protected = github::is_branch_protected(&installation_api, &full_name, &default_branch).await;
        if is_protected {
            push_changes_using_pull_request(path, &full_name, &default_branch).await;
        } else {
            git_util::push(path);
            info!("[update-github-from-crowdin] [{}] pushed", full_name);
        }
    } else {
        info!("[update-github-from-crowdin] [{}] no changes found", full_name);
    }
}

async fn push_changes_using_pull_request(path: &Path, full_name: &str, default_branch: &str) {
    let personal_api = as_personal_account();
    let (owner, repo) = full_name.split_once('/').unwrap();
    if !github::fork_repository(&personal_api, owner, repo).await {
        return;
    }
    let pushed = git_util::push_to_my_fork(path, repo);
    if pushed {
        sleep(Duration::from_secs(30)).await;
        github::create_pull_request(&personal_api, &full_name, &default_branch).await;
        info!("[update-github-from-crowdin] [{}] pushed to crowdin-fml branch and created PR", full_name);
    } else {
        info!("[update-github-from-crowdin] [{}] existing crowdin-fml branch has same content", full_name);
    }
}

async fn move_translated_files_to_mod_directory(mod_directory: &ModDirectory, translation_directory: &Path) {
    delete_unmatched_localization_files(mod_directory);
    for (language_path, language) in util::read_dir(translation_directory) {
        let language_path_crowdin = language_path.join(get_crowdin_directory_name(&mod_directory.github_name));
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

/// Consider:
/// locale/en: ["locale1.cfg"]
/// locale/ru: ["locale1.cfg", "locale2.cfg"]
///
/// Result:
/// locale/en: ["locale1.cfg"]
/// locale/ru: ["locale1.cfg"]
fn delete_unmatched_localization_files(mod_directory: &ModDirectory) {
    let english_files = mod_directory.get_english_files()
        .into_iter()
        .map(|it| util::file_name(&it).to_owned())
        .collect::<HashSet<_>>();
    for (_, localized_files) in mod_directory.get_localizations() {
        for localized_file in localized_files {
            let name = util::file_name(&localized_file);
            if !english_files.contains(name) {
                fs::remove_file(&localized_file).unwrap();
            }
        }
    }
}
