use fml::github;
use fml::mod_directory::ModDirectory;

#[tokio::test]
async fn main() {
    fml::init_with_crowdin().await;

    let api = github::as_app();
    let repositories = github::get_all_repositories(&api).await;
    let mut all_matches = true;
    for (full_name, mods, installation_id) in repositories {
        let repository_directory = github::clone_repository(&full_name, installation_id).await;
        for mod_ in mods {
            let mod_directory = ModDirectory::new(&repository_directory, mod_);
            if !mod_directory.check_for_locale_folder() { continue; }
            if !mod_directory.check_translation_files_match_english_files(false) {
                all_matches = false;
                continue;
            }
        }
    }
    assert!(all_matches);
}
