//! Newlines in translations will break .cfg file format

use std::fs;

use fml::crowdin;
use fml::util::read_dir;

const IGNORED: &[(&str, &str, &str, &str)] = &[
    // Empty translation for unknown reason, Crowdin support doesn't help
    ("Factorio Ultracube (grandseiken)", "de", "tips-and-tricks.ini", "cube-boiler"),
];

#[tokio::test]
async fn main() {
    fml::init_with_crowdin().await;

    let translations_directory = crowdin::download_all_translations().await;
    let mut has_newlines = false;
    // `ru/Factorio Mod Example (dima74)/locale.ini`
    for (language_path, language) in read_dir(translations_directory.path()) {
        for (repository_path, crowdin_name) in read_dir(&language_path) {
            for (file_path, file_name) in read_dir(&repository_path) {
                let content = fs::read_to_string(&file_path).unwrap();
                for line in content.lines() {
                    if let Some(key) = line.strip_suffix('=') {
                        if IGNORED.contains(&(crowdin_name.as_str(), language.as_str(), file_name.as_str(), key)) {
                            continue;
                        }
                        has_newlines = true;
                        eprintln!("[{}] {}/{}: incorrect translation for key {}", crowdin_name, language, file_name, key)
                    }
                }
            }
        }
    }
    assert!(!has_newlines);
}
