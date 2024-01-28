use std::collections::HashSet;

use fml::{crowdin, github};

const IGNORED_GITHUB: &[&str] = &[
    "anyutianluo/factorio-mods-Crowdin-",  // empty repository
];
const IGNORED_CROWDIN: &[&str] = &[
    "Factorio Ntech Chemistry (NathaU)",  // not sure what to do with it
];

#[tokio::test]
async fn main() {
    fml::init_with_crowdin().await;

    let crowdin_names = crowdin::list_directories().await
        .map(|(name, _id)| name)
        .filter(|name| !IGNORED_CROWDIN.contains(&name.as_str()))
        .collect::<HashSet<String>>();

    let mut github = github::as_app();
    let github_names = github::get_all_repositories(&mut github).await
        .into_iter()
        .map(|(full_name, _id)| crowdin::get_crowdin_directory_name(&full_name))
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
    assert_eq!(crowdin_names, github_names, "Crowdin and GitHub names doesn't match");
}
