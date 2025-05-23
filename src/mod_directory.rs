use std::path::{Path, PathBuf};

use log::{error, warn};
use tempfile::TempDir;

use crate::{crowdin, util};
use crate::github_repo_info::GithubModInfo;
use crate::sentry::sentry_report_error;

pub type LanguageCode = String;

/// Represents local directory containing cloned github repository
pub struct RepositoryDirectory {
    /// full name of github repository in format "owner/repo"
    pub full_name: String,
    pub root: TempDir,
}

impl RepositoryDirectory {
    pub fn new(full_name: &str, root: TempDir) -> Self {
        Self {
            full_name: full_name.to_owned(),
            root,
        }
    }
}

/// Represents local directory containing factorio mod
pub struct ModDirectory {
    pub locale_path: PathBuf,
    pub mod_info: GithubModInfo,
}

impl ModDirectory {
    pub fn new(repository_directory: &RepositoryDirectory, mod_info: GithubModInfo) -> Self {
        let repository_root = repository_directory.root.path();
        let locale_path = repository_root.join(&mod_info.locale_path).to_owned();
        Self { locale_path, mod_info }
    }

    pub fn locale_path(&self) -> &Path {
        &self.locale_path
    }

    pub fn locale_en_path(&self) -> PathBuf {
        self.locale_path.join("en")
    }

    pub fn check_structure(&self) -> bool {
        if !self.check_for_locale_folder() {
            warn!("[add-repository] [{}] Missing `locale/en`", &self.mod_info);
            return false;
        }
        
        self.check_translation_files_match_english_files(true)
    }

    pub fn check_for_locale_folder(&self) -> bool {
        self.locale_en_path().exists()
    }

    pub fn check_translation_files_match_english_files(&self, report_sentry: bool) -> bool {
        let localizations = self.get_localizations();
        for (language_code, localized_files) in localizations {
            for localized_file in localized_files {
                let file_name = util::file_name(&localized_file);
                let english_file = self.locale_en_path().join(file_name);
                if !english_file.exists() {
                    let message = format!(
                        "[add-repository] [{}] matched english file not found for '{}/{}'",
                        self.mod_info,
                        language_code,
                        file_name
                    );
                    error!("{}", &message);
                    if report_sentry {
                        sentry_report_error(&message);
                    }
                    return false;
                }
            }
        }
        true
    }

    pub fn get_english_files(&self) -> Vec<PathBuf> {
        util::get_directory_cfg_files_paths(&self.locale_en_path())
    }

    pub fn get_localizations(&self) -> Vec<(LanguageCode, Vec<PathBuf>)> {
        self.get_language_directories()
            .into_iter()
            .filter(|(code, _path)| code != "en")
            .map(|(code, path)| {
                let files = util::get_directory_cfg_files_paths(&path);
                (code, files)
            })
            .collect()
    }

    fn get_language_directories(&self) -> Vec<(LanguageCode, PathBuf)> {
        util::read_dir(self.locale_path())
            .filter(|(path, _name)| path.is_dir())
            .map(|(path, name)| (crowdin::normalize_language_code(&name), path))
            .filter(|(code, _path)| crowdin::is_correct_language_code(code))
            .collect()
    }
}
