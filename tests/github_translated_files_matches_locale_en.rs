//! Checks that on GitHub files in translation directory (e.g. "locale/ru")
//! matches file in english directory ("locale/en").
//! Expected flow:
//! * Mod author deletes english file on GitHub
//! * Our helper deletes corresponding english file on Crowdin
//! * Our helper deletes corresponding translated files on GitHub
//! 
//! Last step happens with delays (one-week sync delay and delay for merging PR),
//! therefor this test is ignored on CI.

use fml::github;
use fml::mod_directory::ModDirectory;

#[ignore]
#[tokio::test]
async fn main() {
    // Disable info logging in `github::clone_repository`
    std::env::set_var("RUST_LOG", "fml=warn");
    fml::init();

    let api = github::as_app();
    let repositories = github::get_all_repositories(&api).await;
    let mut all_matches = true;
    for (repo_info, installation_id) in repositories {
        let repository_directory = github::clone_repository(&repo_info, installation_id).await;
        for mod_ in repo_info.mods {
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
