use std::path::{Path, PathBuf};

use log::error;
use tempfile::TempDir;

use crate::{crowdin, util};

pub type LanguageCode = String;

/// Represents local copy of github mod repository
pub struct ModDirectory {
    /// full name of github repository
    pub github_full_name: String,
    root: TempDir,
}

impl ModDirectory {
    pub fn new(full_name: &str, root: TempDir) -> Self {
        Self {
            github_full_name: full_name.to_owned(),
            root,
        }
    }

    pub fn root(&self) -> &Path {
        self.root.path()
    }

    pub fn locale_path(&self) -> PathBuf {
        self.root.path().join("locale")
    }

    pub fn locale_en_path(&self) -> PathBuf {
        self.root.path().join("locale/en")
    }

    pub fn check_for_locale_folder(&self) -> bool {
        if self.locale_en_path().exists() {
            true
        } else {
            error!("[add-repository] [{}] '/locale/en' subdirectory not found in github repository", self.github_full_name);
            false
        }
    }

    pub fn check_translation_files_match_english_files(&self) {
        let localizations = self.get_localizations();
        for (language_code, localized_files) in localizations {
            for localized_file in localized_files {
                let file_name = util::file_name(&localized_file);
                let english_file = self.locale_en_path().join(file_name);
                if !english_file.exists() {
                    panic!("[add-repository] [{}] matched english file not found for '{}/{}'", self.github_full_name, language_code, file_name);
                }
            }
        }
    }

    pub fn get_english_files(&self) -> Vec<PathBuf> {
        util::get_directory_cfg_files_paths(&self.locale_en_path())
    }

    pub fn get_localizations(&self) -> Vec<(LanguageCode, Vec<PathBuf>)> {
        self.get_language_codes()
            .into_iter()
            .filter(|code| code != "en")
            .map(|code| {
                let path = self.locale_path().join(&code);
                let files = util::get_directory_cfg_files_paths(&path);
                (code, files)
            })
            .collect()
    }

    fn get_language_codes(&self) -> Vec<String> {
        util::read_dir(&self.locale_path())
            .filter(|(path, _name)| path.is_dir())
            .map(|(_path, name)| crowdin::normalize_language_code(&name))
            .filter(|code| crowdin::is_correct_language_code(code))
            .collect()
    }
}
