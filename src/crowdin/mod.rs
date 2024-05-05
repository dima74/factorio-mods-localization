use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::Deref;
use std::path::Path;
use std::sync::{LazyLock, OnceLock};
use std::time::Duration;

use log::info;
use octocrab::models::InstallationId;
use regex::Regex;
use serde::{Deserialize, Serialize};
use tempfile::TempDir;

use crate::crowdin::http::{crowdin_get_empty_query, crowdin_get_pagination, crowdin_get_pagination_empty_query, crowdin_post, crowdin_put, DataWrapper, IdResponse, UnitResponse, upload_file_to_storage};
use crate::github_repo_info::{GithubModName, GithubRepoInfo};
use crate::mod_directory::{LanguageCode, ModDirectory};
use crate::myenv::is_development;
use crate::util;

pub mod http;

pub static PROJECT_LANGUAGE_CODES: OnceLock<Vec<String>> = OnceLock::new();

pub async fn init() {
    let info = get_project_info().await;
    if !is_development() {
        assert_eq!(info.name, "Factorio mods localization");
    }

    let codes = info.target_language_ids;
    assert!(codes.len() > 20);
    PROJECT_LANGUAGE_CODES.set(codes).unwrap();
}

type DirectoryId = i64;
/// id of english file
type FileId = i64;
/// id of english/localized file in storage
type StorageId = i64;

#[derive(Deserialize)]
struct ProjectInfo {
    #[serde(rename = "targetLanguageIds")]
    target_language_ids: Vec<String>,
    name: String,
}

// https://developer.crowdin.com/api/v2/#operation/api.projects.get
async fn get_project_info() -> ProjectInfo {
    crowdin_get_empty_query("").await
}

pub async fn find_directory_id(crowdin_name: &str) -> Option<DirectoryId> {
    list_directories().await
        .find(|(name, _id)| name == crowdin_name)
        .map(|(_name, id)| id)
}

// https://developer.crowdin.com/api/v2/#operation/api.projects.directories.getMany
pub async fn list_directories() -> impl Iterator<Item=(String, DirectoryId)> {
    #[derive(Deserialize)]
    struct Directory { id: DirectoryId, name: String }
    let directories: Vec<DataWrapper<Directory>> = crowdin_get_pagination_empty_query("/directories").await;
    directories.into_iter().map(|d| (d.data.name, d.data.id))
}

// https://developer.crowdin.com/api/v2/#operation/api.projects.files.getMany
pub async fn list_files(directory_id: DirectoryId) -> impl Iterator<Item=(String, FileId)> {
    #[derive(Deserialize)]
    struct File { id: FileId, name: String }
    let files: Vec<DataWrapper<File>> = crowdin_get_pagination("/files", |request| {
        request.query(&[("directoryId", directory_id)])
    }).await;
    files.into_iter().map(|d| (d.data.name, d.data.id))
}

// https://developer.crowdin.com/api/v2/#operation/api.projects.directories.post
pub async fn create_directory(name: &str) -> DirectoryId {
    #[derive(Serialize)]
    struct Request<'a> { name: &'a str }
    let request = Request { name };
    crowdin_post::<_, IdResponse>("/directories", request).await.id
}

pub async fn filter_repositories(
    repositories: Vec<(GithubRepoInfo, InstallationId)>
) -> Vec<(GithubRepoInfo, InstallationId)> {
    let directories = list_directories().await
        .map(|(name, _id)| name)
        .collect::<HashSet<_>>();
    repositories
        .into_iter()
        .filter_map(|(repo_info, api)| {
            let repo_info = repo_info.filter_mods_present_on_crowdin(&directories)?;
            Some((repo_info, api))
        })
        .collect()
}

// https://developer.crowdin.com/api/v2/#operation/api.projects.files.post
async fn add_english_file(directory_id: DirectoryId, storage_id: StorageId, file_name: &str) -> FileId {
    #[derive(Serialize)]
    struct Request<'a> {
        #[serde(rename = "directoryId")]
        directory_id: DirectoryId,
        #[serde(rename = "storageId")]
        storage_id: StorageId,
        #[serde(rename = "name")]
        file_name: &'a str,
        r#type: &'static str,
    }
    let request = Request { directory_id, storage_id, file_name, r#type: "ini" };
    crowdin_post::<_, IdResponse>("/files", request).await.id
}

// https://developer.crowdin.com/api/v2/#operation/api.projects.files.put
async fn update_english_file(file_id: FileId, storage_id: StorageId) {
    #[derive(Serialize)]
    struct Request {
        #[serde(rename = "storageId")]
        storage_id: StorageId,
    }
    let request = Request { storage_id };
    let method = format!("/files/{}", file_id);
    crowdin_put::<_, UnitResponse>(&method, request).await;
}

// https://developer.crowdin.com/api/v2/#operation/api.projects.translations.postOnLanguage
pub async fn add_localization_file(
    // id of source english file
    english_file_id: FileId,
    // id of localized file
    storage_id: StorageId,
    language_code: &LanguageCode,
) {
    #[derive(Serialize)]
    struct Request {
        #[serde(rename = "fileId")]
        english_file_id: FileId,
        #[serde(rename = "storageId")]
        storage_id: StorageId,
        /// Defines whether to add translation if it's the same as the source string
        #[serde(rename = "importEqSuggestions")]
        import_eq_suggestions: bool,
        /// Mark uploaded translations as approved
        #[serde(rename = "autoApproveImported")]
        auto_approve_imported: bool,
    }
    let request = Request {
        english_file_id,
        storage_id,
        import_eq_suggestions: false,
        auto_approve_imported: false,
    };
    let path = format!("/translations/{}", language_code);
    crowdin_post::<_, UnitResponse>(&path, request).await;
}

pub async fn download_all_translations() -> TempDir {
    // https://developer.crowdin.com/api/v2/#operation/api.projects.translations.builds.post
    async fn build_translations() -> i64 {
        #[derive(Serialize)]
        struct Request {
            #[serde(rename = "skipUntranslatedStrings")]
            skip_untranslated_strings: bool,
        }
        let request = Request { skip_untranslated_strings: true };
        crowdin_post::<_, IdResponse>("/translations/builds", request).await.id
    }
    // https://developer.crowdin.com/api/v2/#operation/api.projects.translations.builds.get
    async fn is_build_finished(build_id: i64) -> bool {
        #[derive(Deserialize)]
        struct Response { status: String }
        let url = format!("/translations/builds/{}", build_id);
        let response = crowdin_get_empty_query::<Response>(&url).await;
        response.status != "inProgress"
    }
    // https://developer.crowdin.com/api/v2/#operation/api.projects.translations.builds.download.download
    async fn get_build_download_url(build_id: i64) -> String {
        #[derive(Deserialize)]
        struct Response { url: String }
        let url = format!("/translations/builds/{}/download", build_id);
        crowdin_get_empty_query::<Response>(&url).await.url
    }

    let build_id = build_translations().await;
    while !is_build_finished(build_id).await {
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    let url = get_build_download_url(build_id).await;
    let result = util::download_and_extract_zip_file(&url).await;
    util::remove_empty_ini_files(result.path());
    result
}

// https://developer.crowdin.com/api/v2/#operation/api.projects.files.get
pub async fn download_file(file_id: FileId) -> fs::File {
    #[derive(Deserialize)]
    struct Response { url: String }
    let url = format!("/files/{}/download", file_id);
    let response: Response = crowdin_get_empty_query(&url).await;
    util::download_file(&response.url).await
}

pub struct CrowdinDirectory {
    crowdin_id: DirectoryId,
    #[allow(unused)]
    crowdin_name: String,
    pub mod_directory: ModDirectory,
}

impl CrowdinDirectory {
    pub async fn get_or_create(mod_directory: ModDirectory) -> (CrowdinDirectory, bool) {
        let crowdin_name = get_crowdin_directory_name(&mod_directory.github_name);
        let (crowdin_id, created) = match find_directory_id(&crowdin_name).await {
            Some(crowdin_id) => (crowdin_id, false),
            None => (create_directory(&crowdin_name).await, true),
        };
        (Self { crowdin_id, crowdin_name, mod_directory }, created)
    }

    pub async fn has_existing(mod_directory: &ModDirectory) -> bool {
        let crowdin_name = get_crowdin_directory_name(&mod_directory.github_name);
        find_directory_id(&crowdin_name).await.is_some()
    }

    pub async fn add_english_and_localization_files(&self) {
        let english_file_ids = self.add_english_files().await;
        self.add_localization_files(english_file_ids).await;
    }

    pub async fn add_english_files(&self) -> HashMap<String, FileId> {
        let existing_crowdin_files: HashMap<String, FileId> = list_files(self.crowdin_id).await.collect();
        let mut result = HashMap::new();
        for file_path in self.mod_directory.get_english_files() {
            let file_name_ini = replace_cfg_to_ini(util::file_name(&file_path));
            let file_id = self.add_or_update_english_file(&existing_crowdin_files, &file_path, &file_name_ini).await;
            result.insert(file_name_ini, file_id);
        }
        result
    }

    async fn add_or_update_english_file(
        &self,
        existing_crowdin_files: &HashMap<String, FileId>,
        file_path: &Path,
        file_name_ini: &str,
    ) -> FileId {
        match existing_crowdin_files.get(file_name_ini) {
            Some(&file_id) => {
                self.update_english_file(file_id, file_path, file_name_ini).await;
                file_id
            }
            None => {
                self.add_english_file(file_path, file_name_ini).await
            }
        }
    }

    async fn add_english_file(&self, file: &Path, file_name: &str) -> FileId {
        let storage_id = self.upload_file_to_storage(file, file_name).await;
        add_english_file(self.crowdin_id, storage_id, file_name).await
    }

    async fn update_english_file(&self, file_id: FileId, file: &Path, file_name: &str) {
        let storage_id = self.upload_file_to_storage(file, file_name).await;
        update_english_file(file_id, storage_id).await;
    }

    async fn add_localization_files(&self, english_file_ids: HashMap<String, FileId>) {
        for (language_code, files) in self.mod_directory.get_localizations() {
            for file in files {
                let file_name = replace_cfg_to_ini(util::file_name(&file));
                let english_file_id = english_file_ids[&file_name];
                self.add_localization_file(&file, &file_name, english_file_id, &language_code).await;
            }
        }
    }

    async fn add_localization_file(
        &self,
        file: &Path,
        file_name: &str,
        english_file_id: FileId,
        language_code: &LanguageCode,
    ) {
        let storage_id = self.upload_file_to_storage(file, file_name).await;
        add_localization_file(english_file_id, storage_id, language_code).await;
    }

    async fn upload_file_to_storage(&self, file: &Path, file_name: &str) -> StorageId {
        info!("[{}] upload file to storage: {}/{}", self.mod_directory.github_name, util::file_name(file.parent().unwrap()), file_name);
        let file_content = fs::read_to_string(file).unwrap();
        let mut file_content = util::escape::escape_strings_in_ini_file(&file_content);
        if file_content.is_empty() {
            file_content = "; empty".to_owned();
        }
        upload_file_to_storage(file_content, file_name).await
    }
}

/// crowdin expects codes in format 'pt-BR'
/// however some mods use 'pt-br' as language code
/// (e.g. https://github.com/JonasJurczok/factorio-todo-list/tree/master/locale/pt-br)
/// this function converts 'pt-br' to 'pt-BR'
pub fn normalize_language_code(code: &str) -> String {
    match code.split_once('-') {
        None => code.to_ascii_lowercase(),
        Some((part1, part2)) => {
            format!("{}-{}", part1.to_ascii_lowercase(), part2.to_ascii_uppercase())
        }
    }
}

pub fn is_correct_language_code(code: &str) -> bool {
    let codes = PROJECT_LANGUAGE_CODES.get().unwrap();
    codes.iter().any(|it| it == code)
}

pub fn get_crowdin_directory_name(github_name: &GithubModName) -> String {
    use util::case::to_title_case;
    let repo = to_title_case(&github_name.repo);
    match &github_name.subpath {
        None => {
            format!("{} ({})", repo, github_name.owner)
        }
        Some(subpath) => {
            format!("{} - {} ({})", repo, to_title_case(subpath), github_name.owner)
        }
    }
}

pub fn replace_cfg_to_ini(name: &str) -> String {
    static DOT_CFG_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(".cfg$").unwrap());
    name.replace(DOT_CFG_REGEX.deref(), ".ini")
}

pub fn replace_ini_to_cfg(name: &str) -> String {
    static DOT_INI_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(".ini$").unwrap());
    name.replace(DOT_INI_REGEX.deref(), ".cfg")
}
