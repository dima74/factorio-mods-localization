use std::path::Path;

use fml::github::get_all_repositories;

const USE_CACHED: bool = false;

#[tokio::main]
async fn main() {
    fml::init();
    let cache_path = Path::new("temp/repositories.json");
    let repositories: Vec<String> = if USE_CACHED {
        let json = std::fs::read_to_string(cache_path).unwrap();
        serde_json::from_str(&json).unwrap()
    } else {
        let api = fml::github::as_app();
        let repositories = get_all_repositories(&api).await
            .into_iter()
            .map(|(repository, _repo_info, _id)| repository)
            .collect::<Vec<_>>();

        let json = serde_json::to_string_pretty(&repositories).unwrap();
        std::fs::write(cache_path, json).unwrap();

        repositories
    };

    for repository in repositories {
        println!("git clone --depth 1 git@github.com:{}.git &", repository);
    }
}
