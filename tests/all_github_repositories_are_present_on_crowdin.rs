use std::collections::HashSet;
use std::ops::Deref;

use fml::{crowdin, github};

const EXCLUDED: &[&str] = &["anyutianluo/factorio-mods-Crowdin-"];

#[tokio::test]
async fn main() {
    fml::init_with_crowdin().await;

    let crowdin_names: HashSet<String> = crowdin::list_directories().await
        .map(|(name, _id)| name).collect();

    let mut ok = true;
    let mut github = github::as_app();
    let repositories = github::get_all_repositories(&mut github).await;
    for (full_name, _) in repositories {
        if EXCLUDED.contains(&full_name.deref()) { continue; }
        let crowdin_name = crowdin::get_crowdin_directory_name(&full_name);
        if !crowdin_names.contains(&crowdin_name) {
            println!("github name '{}' vs non-existent crowdin name '{}'", &full_name, &crowdin_name);
            ok = false;
        }
    }
    assert!(ok);
}
