use std::collections::HashSet;

use fml::{crowdin, github};
use fml::crowdin::get_crowdin_directory_name;

const IGNORED_GITHUB: &[&str] = &[];
const IGNORED_CROWDIN: &[&str] = &[
    // Used for testing
    "Factorio Mod Example (dima74)",
    // github repository deleted or hidden, but mod page still has link to crowdin, so keep for now
    "Factorio Ntech Chemistry (NathaU)",
];

#[tokio::test]
async fn main() {
    fml::init_with_crowdin().await;

    let crowdin_names = crowdin::list_directories().await
        .map(|(name, _id)| name)
        .filter(|name| !IGNORED_CROWDIN.contains(&name.as_str()))
        .collect::<HashSet<String>>();

    let api = github::as_app();
    let github_names = github::get_all_repositories(&api).await
        .into_iter()
        .flat_map(|(repo_info, _id)| repo_info.mods)
        .map(|it| get_crowdin_directory_name(&it))
        .filter(|name| !IGNORED_GITHUB.contains(&name.as_str()))
        .collect::<HashSet<String>>();

    for name in &github_names {
        if !crowdin_names.contains(name) {
            println!("Missing on crowdin: '{}'", name)
        }
    }
    for name in &crowdin_names {
        if !github_names.contains(name) {
            println!("Extra on crowdin: '{}'", name)
        }
    }
    if crowdin_names != github_names {
        panic!("Crowdin and GitHub names doesn't match");
    }
}
