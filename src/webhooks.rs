use std::ops::Deref;

use InstallationWebhookEventAction::{Created, Deleted};
use log::info;
use octocrab::models::InstallationId;
use octocrab::models::webhook_events::{EventInstallation, InstallationEventRepository, WebhookEvent, WebhookEventPayload};
use octocrab::models::webhook_events::payload::{InstallationWebhookEventAction, PushWebhookEventPayload};
use WebhookEventPayload::{Installation, InstallationRepositories, Push};

use crate::{github, util};
use crate::crowdin::CrowdinDirectory;
use crate::github::filter_repositories_with_locale_en;

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
    let has_repository_selection_all = util::has_repository_selection_all(installation.repository_selection.as_ref().unwrap());
    on_repositories_added(
        repositories,
        installation_id,
        has_repository_selection_all,
    ).await;
    info!("[installation-webhook] [{}] success", user);
}

async fn on_repositories_added(
    repositories: Vec<InstallationEventRepository>,
    installation_id: InstallationId,
    has_repository_selection_all: bool,
) {
    let repositories = repositories
        .into_iter()
        .filter(|it| !it.private)
        .map(|it| it.full_name)
        .collect::<Vec<_>>();
    let api = github::as_installation(installation_id);
    let repositories = if has_repository_selection_all {
        filter_repositories_with_locale_en(&api, repositories).await
    } else {
        repositories
    };
    for repository in repositories {
        on_repository_added(&repository, installation_id).await;

        let api_personal = github::as_personal_account();
        github::star_repository(&api_personal, &repository).await;
    }
}

pub async fn on_repository_added(full_name: &str, installation_id: InstallationId) {
    info!("[email] app installed for repository {}", full_name);
    let mod_directory = github::clone_repository(&full_name, installation_id).await;
    if !mod_directory.check_for_locale_folder() { return; }
    mod_directory.check_translation_files_match_english_files();

    let (crowdin_directory, _) = CrowdinDirectory::get_or_create(mod_directory).await;
    crowdin_directory.on_repository_added().await;
    info!("[add-repository] [{}] success", full_name);
}

pub async fn on_push_event(
    event: &PushWebhookEventPayload,
    installation_id: InstallationId,
    full_name: String,
) {
    info!("\n[push-webhook] [{}] starting...", full_name);
    let modified_files = get_all_modified_and_added_files(&event);
    let modified_locale_en_files: Vec<&str> = modified_files
        .filter_map(|file| file.strip_prefix("locale/en/"))
        .filter(|file| file.ends_with(".cfg") && !file.contains('/'))
        .collect();
    if modified_locale_en_files.is_empty() {
        info!("[push-webhook] [{}] no modified/added english files found", full_name);
        return;
    }

    let mod_directory = github::clone_repository(&full_name, installation_id).await;
    if !mod_directory.check_for_locale_folder() { return; }

    let (crowdin_directory, created) = CrowdinDirectory::get_or_create(mod_directory).await;
    if created {
        info!("[push-webhook] [{}] created directory on crowdin - performing full import", full_name);
        crowdin_directory.on_repository_added().await;
    } else {
        crowdin_directory.update_english_files(&modified_locale_en_files).await;
    }
    info!("[push-webhook] [{}] success", full_name);
}

fn get_all_modified_and_added_files(event: &PushWebhookEventPayload) -> impl Iterator<Item=&str> {
    event.commits.iter()
        .flat_map(|commit| {
            let added = commit.added.iter();
            let modified = commit.modified.iter();
            added.chain(modified).map(Deref::deref)
        })
}
