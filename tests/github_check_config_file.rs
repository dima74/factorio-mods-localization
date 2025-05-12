//! Checks `factorio-mods-localization.json` config file.
//! Repository either should not have it, or have it correct.

use fml::github;
use fml::github::{get_all_installations, get_all_repositories_of_installation, get_repo_info, GetRepoInfoError};

#[tokio::test]
async fn main() {
    fml::init();
    let api = github::as_app();

    let mut repos_with_invalid_config = Vec::new();
    let installations = get_all_installations(&api).await;
    for installation in installations {
        let installation_api = api.installation(installation.id);
        let repositories = get_all_repositories_of_installation(&installation_api).await;
        for repository in repositories {
            let repo_info = get_repo_info(&installation_api, &repository).await;
            if let Err(GetRepoInfoError::InvalidConfig) = repo_info {
                repos_with_invalid_config.push(repository);
            }
        }
    }

    if !repos_with_invalid_config.is_empty() {
        eprintln!("\n\nFound {} repositories with invalid config:", repos_with_invalid_config.len());
        for repo in repos_with_invalid_config {
            eprintln!("{repo}");
        }
        eprintln!("\n");

        panic!("There are repositories with invalid config.");
    }
}
