use std::collections::{HashMap, HashSet};

use octocrab::Octocrab;

use fml::{crowdin, github};
use fml::crowdin::get_crowdin_directory_name;
use fml::github_repo_info::GithubModName;

#[tokio::test]
async fn main() {
    fml::init_with_crowdin().await;
    let github_data = get_github_data().await;
    let crowdin_data = get_crowdin_data().await;

    let mut matches = true;
    for (crowdin_name, ref crowdin_files) in crowdin_data {
        let Some(github_files) = github_data.get(&crowdin_name) else { continue; };

        for file in github_files {
            if !crowdin_files.contains(file) {
                println!("[{}] Missing on crowdin: '{}'", crowdin_name, file)
            }
        }
        for files in crowdin_files {
            if !github_files.contains(files) {
                println!("[{}] Extra on crowdin: '{}'", crowdin_name, files)
            }
        }
        matches &= crowdin_files == github_files;
    }
    assert!(matches, "Crowdin and GitHub names doesn't match");
}

async fn get_crowdin_data() -> HashMap<String, HashSet<String>> {
    let mut result = HashMap::new();
    let directories = crowdin::list_directories().await;
    for (crowdin_name, directory_id) in directories {
        let files = crowdin::list_files(directory_id).await;
        let files = files
            .map(|(name, _)| crowdin::replace_ini_to_cfg(&name))
            .collect();
        result.insert(crowdin_name, files);
    }
    result
}

async fn get_github_data() -> HashMap<String, HashSet<String>> {
    let api = github::as_app();
    let repositories = github::get_all_repositories(&api).await;
    let mut result = HashMap::new();
    for (repo_info, installation_id) in repositories {
        for mod_ in repo_info.mods {
            let installation_api = api.installation(installation_id);
            let files = list_locale_en_files_for_mod(&repo_info.full_name, &mod_, &installation_api).await;
            let files = match files {
                Some(value) => value,
                None => continue,
            };
            let crowdin_name = get_crowdin_directory_name(&mod_);
            result.insert(crowdin_name, files);
        }
    }
    result
}

async fn list_locale_en_files_for_mod(
    full_name: &str,
    mod_: &GithubModName,
    installation_api: &Octocrab,
) -> Option<HashSet<String>> {
    let path = match mod_.subpath {
        None => "locale/en".to_owned(),
        Some(ref subpath) => format!("{}/locale/en", subpath),
    };
    let files = github::list_files_in_directory(&installation_api, &full_name, &path).await.ok()?;
    let files = files
        .into_iter()
        .filter(|name| name.ends_with(".cfg"))
        .collect();
    Some(files)
}
