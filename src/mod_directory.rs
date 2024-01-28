use std::path::{Path, PathBuf};

use log::error;
use sentry::Level;
use tempfile::TempDir;

use crate::{crowdin, util};
use crate::github::GITHUB_MODS_FILE_NAME;
use crate::github_mod_name::{GithubModName, parse_github_mod_names_json};

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

    pub fn get_mods(&self) -> Vec<GithubModName> {
        get_mods_impl(&self.full_name, self.root.path())
    }
}

pub fn get_mods_impl(full_name: &str, root: &Path) -> Vec<GithubModName> {
    let mods_file = root.join(GITHUB_MODS_FILE_NAME);
    if mods_file.exists() {
        let json = std::fs::read_to_string(mods_file).unwrap();
        parse_github_mod_names_json(full_name, &json)
    } else {
        // Usual case - single mod at root of the github repository
        vec![GithubModName::new(full_name, None)]
    }
}

/// Represents local directory containing factorio mod
pub struct ModDirectory {
    pub root: PathBuf,
    pub github_name: GithubModName,
}

impl ModDirectory {
    pub fn new(repository_directory: &RepositoryDirectory, github_name: GithubModName) -> Self {
        let repository_root = repository_directory.root.path();
        let root = match &github_name.subpath {
            None => repository_root.to_owned(),
            Some(subpath) => repository_root.join(subpath).to_owned(),
        };
        Self { root, github_name }
    }

    pub fn locale_path(&self) -> PathBuf {
        self.root.join("locale")
    }

    pub fn locale_en_path(&self) -> PathBuf {
        self.root.join("locale/en")
    }

    pub fn check_structure(&self) -> bool {
        self.check_for_locale_folder() && self.check_translation_files_match_english_files()
    }

    fn check_for_locale_folder(&self) -> bool {
        if self.locale_en_path().exists() {
            true
        } else {
            error!("[add-repository] [{}] '/locale/en' subdirectory not found in github repository", self.github_name);
            false
        }
    }

    fn check_translation_files_match_english_files(&self) -> bool {
        let localizations = self.get_localizations();
        for (language_code, localized_files) in localizations {
            for localized_file in localized_files {
                let file_name = util::file_name(&localized_file);
                let english_file = self.locale_en_path().join(file_name);
                if !english_file.exists() {
                    let message = format!("[add-repository] [{}] matched english file not found for '{}/{}'", self.github_name, language_code, file_name);
                    sentry::capture_message(&message, Level::Error);
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
