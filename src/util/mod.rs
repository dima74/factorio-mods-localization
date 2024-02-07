use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};

use tempfile::TempDir;

pub mod case;
pub mod escape;

pub type UnitResult = Result<(), Box<dyn Error>>;

pub fn read_dir(path: &Path) -> impl Iterator<Item=(PathBuf, String)> {
    fs::read_dir(path).unwrap()
        .map(|entry| {
            let path = entry.unwrap().path();
            let name = file_name(&path).to_owned();
            (path, name)
        })
}

pub fn file_name(path: &Path) -> &str {
    path.file_name().unwrap().to_str().unwrap()
}

pub fn get_directory_cfg_files_paths(path: &Path) -> Vec<PathBuf> {
    read_dir(path)
        .filter(|(path, name)| path.is_file() && name.ends_with(".cfg"))
        .map(|(path, _name)| path)
        .collect()
}

pub async fn download_and_extract_zip_file(url: &str) -> TempDir {
    use zip::ZipArchive;

    let file = download_file(url).await;
    let mut zip = ZipArchive::new(file).unwrap();
    let directory = TempDir::with_prefix("FML.").unwrap();
    zip.extract(&directory).unwrap();
    directory
}

pub async fn download_file(url: &str) -> File {
    let file = tempfile::tempfile().unwrap();
    let mut file = tokio::fs::File::from_std(file);
    let response = reqwest::get(url)
        .await.unwrap()
        .bytes().await.unwrap();
    tokio::io::copy(&mut &*response, &mut file).await.unwrap();
    file.into_std().await
}

pub fn remove_empty_ini_files(root: &Path) {
    // `ru/Factorio Mod Example (dima74)/test.ini`
    for (language_path, _) in read_dir(root) {
        for (repository_path, _) in read_dir(&language_path) {
            for (file_path, _) in read_dir(&repository_path) {
                let content = fs::read_to_string(&file_path).unwrap();
                if is_empty_ini_file(content) {
                    fs::remove_file(file_path).unwrap();
                }
            }
        }
    }
}

fn is_empty_ini_file(content: String) -> bool {
    content.lines().all(|line| {
        let line = line.trim();
        line.is_empty() || line.starts_with('[') || line.starts_with(';')
    })
}

#[derive(Debug)]
pub struct EmptyBody;

#[async_trait::async_trait]
impl ::octocrab::FromResponse for EmptyBody {
    async fn from_response(_: ::http::Response<::hyper::Body>) -> ::octocrab::Result<Self> {
        Ok(Self)
    }
}
