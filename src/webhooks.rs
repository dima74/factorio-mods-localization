use std::ops::Deref;

use InstallationWebhookEventAction::{Created, Deleted};
use log::info;
use octocrab::models::InstallationId;
use octocrab::models::webhook_events::{EventInstallation, InstallationEventRepository, WebhookEvent, WebhookEventPayload};
use octocrab::models::webhook_events::payload::{InstallationWebhookEventAction, PushWebhookEventPayload};
use WebhookEventPayload::{Installation, InstallationRepositories, Push};

use crate::crowdin::CrowdinDirectory;
use crate::github;
use crate::github::GITHUB_CONFIG_FILE_NAME;
use crate::github_repo_info::GithubRepoInfo;
use crate::mod_directory::ModDirectory;

pub async fn webhook_impl(event: WebhookEvent) {
    match event.specific {
        Installation(_) | InstallationRepositories(_) => {
            handle_installation_event(event).await;
        }
        Push(payload) => {
            let EventInstallation::Minimal(installation) = event.installation.as_ref().unwrap() else {
                panic!("Unexpected installation data");
            };
            let full_name = event.repository.unwrap().full_name.unwrap();
            on_push_event(&payload, installation.id, full_name).await;
        }
        // TODO InstallationTarget event (user changed login)
        _ => info!("[webhook] unknown event: {:?}", event.kind),
    };
}

async fn handle_installation_event(event: WebhookEvent) {
    let user = event.sender.unwrap().login;
    let EventInstallation::Full(installation) = event.installation.as_ref().unwrap() else {
        panic!("Unexpected installation data");
    };
    let repositories = match event.specific {
        Installation(payload) => {
            match payload.action {
                Created => {
                    payload.repositories.unwrap()
                }
                Deleted => {
                    info!("[email] app uninstalled for user {}", user);
                    return;
                }
                _ => {
                    info!("[installation-webhook] [{}] unknown action: {:?}", user, payload.action);
                    return;
                }
            }
        }
        InstallationRepositories(payload) => {
            for repository_removed in payload.repositories_removed {
                info!("[email] app uninstalled for repository {}", repository_removed.full_name);
            }
            payload.repositories_added
        }
        _ => panic!("Unexpected event type"),
    };

    info!("\n[installation-webhook] [{}] starting for {} repositories...", user, repositories.len());
    let installation_id = installation.id;
    on_repositories_added(repositories, installation_id).await;
    info!("[installation-webhook] [{}] success", user);
}

async fn on_repositories_added(repositories: Vec<InstallationEventRepository>, installation_id: InstallationId) {
    let repositories = repositories
        .into_iter()
        .filter(|it| !it.private)
        .map(|it| it.full_name)
        .collect::<Vec<_>>();
    let api = github::as_installation(installation_id);
    for repository in repositories {
        let Some(repo_info) = github::get_repo_info(&api, &repository).await else {
            continue;
        };
        on_repository_added(repo_info, installation_id).await;

        let api_personal = github::as_personal_account();
        github::star_repository(&api_personal, &repository).await;
    }
}

pub async fn on_repository_added(repo_info: GithubRepoInfo, installation_id: InstallationId) {
    info!("[email] app installed for repository {}", repo_info.full_name);
    let repository_directory = github::clone_repository(&repo_info, installation_id).await;
    for mod_ in repo_info.mods {
        let mod_directory = ModDirectory::new(&repository_directory, mod_);
        if !mod_directory.check_structure() { continue; }

        let (crowdin_directory, _) = CrowdinDirectory::get_or_create(mod_directory).await;
        crowdin_directory.add_english_and_localization_files().await;
    }
    info!("[add-repository] [{}] success", repo_info.full_name);
}

pub async fn import_english(repo_info: GithubRepoInfo, installation_id: InstallationId) {
    let repository_directory = github::clone_repository(&repo_info, installation_id).await;
    for mod_ in repo_info.mods {
        let mod_directory = ModDirectory::new(&repository_directory, mod_);
        if !mod_directory.check_for_locale_folder() { continue; }

        if !CrowdinDirectory::has_existing(&mod_directory).await { continue; }
        let (crowdin_directory, _) = CrowdinDirectory::get_or_create(mod_directory).await;
        crowdin_directory.add_english_files().await;
    }
}

pub async fn on_push_event(
    event: &PushWebhookEventPayload,
    installation_id: InstallationId,
    full_name: String,
) {
    info!("\n[push-webhook] [{}] starting...", full_name);

    if !has_interesting_changes(&event) {
        info!("[push-webhook] [{}] no modified/added english files found", full_name);
        return;
    };

    let api = github::as_installation(installation_id);
    let Some(repo_info) = github::get_repo_info(&api, &full_name).await else {
        info!("[push-webhook] [{}] no mods found", full_name);
        return;
    };

    let repository_directory = github::clone_repository(&repo_info, installation_id).await;
    let mut created = false;
    for mod_ in repo_info.mods {
        let mod_directory = ModDirectory::new(&repository_directory, mod_);
        if !mod_directory.check_for_locale_folder() { continue; }
        created |= handle_push_event_for_mod(mod_directory).await;
    }
    info!("[push-webhook] [{}] success", full_name);

    if created {
        let api_personal = github::as_personal_account();
        github::star_repository(&api_personal, &full_name).await;
    }
}

async fn handle_push_event_for_mod(mod_directory: ModDirectory) -> bool {
    let exists = CrowdinDirectory::has_existing(&mod_directory).await;
    if !exists && !mod_directory.check_translation_files_match_english_files(true) {
        return false;
    }

    let (crowdin_directory, created) = CrowdinDirectory::get_or_create(mod_directory).await;
    if created {
        info!("[push-webhook] [{}] created directory on crowdin - performing full import", crowdin_directory.mod_directory.github_name);
        crowdin_directory.add_english_and_localization_files().await;
    } else {
        crowdin_directory.add_english_files().await;
    }
    created
}

fn has_interesting_changes(event: &PushWebhookEventPayload) -> bool {
    let mut changed_files = get_all_changed_files(&event);
    changed_files.any(|file| {
        file == GITHUB_CONFIG_FILE_NAME || file.contains("locale/en/")
    })
}

fn get_all_changed_files(event: &PushWebhookEventPayload) -> impl Iterator<Item=&str> {
    event.commits.iter()
        .flat_map(|commit| {
            let added = commit.added.iter();
            let modified = commit.modified.iter();
            let removed = commit.removed.iter();
            added.chain(modified).chain(removed).map(Deref::deref)
        })
}
