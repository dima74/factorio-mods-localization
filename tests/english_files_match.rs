/// Creates directories `temp/compare/english-crowdin` and `temp/compare/english-github`.
/// After you should manually execute `diff -r` on them.

use std::{fs, io};
use std::fs::File;
use std::io::{Seek, SeekFrom};
use std::path::Path;

use fml::{crowdin, util};
use fml::crowdin::{get_crowdin_directory_name, replace_cfg_to_ini};
use fml::util::escape::escape_strings_in_ini_file;

#[ignore]  // Ignore on CI
#[tokio::test]
async fn main() {
    fml::init_with_crowdin().await;
    download_crowdin_files().await;
    download_github_files().await;
}

async fn download_crowdin_files() {
    let root = Path::new("temp/compare/english-crowdin");
    if root.exists() {
        panic!("Delete existing {:?}", root);
    }
    fs::create_dir_all(root).unwrap();

    let directories = crowdin::list_directories().await;
    for (directory_name, directory_id) in directories {
        let directory_path = root.join(&directory_name);
        fs::create_dir(&directory_path).unwrap();
        for (file_name, file_id) in crowdin::list_files(directory_id).await {
            let mut file = crowdin::download_file(file_id).await;
            file.seek(SeekFrom::Start(0)).unwrap();
            let target = directory_path.join(file_name);
            let mut target = File::create(target).unwrap();
            io::copy(&mut file, &mut target).unwrap();
        }
    }
}

async fn download_github_files() {
    let target_root = Path::new("temp/compare/english-github");
    if target_root.exists() {
        panic!("Delete existing {:?}", target_root);
    }
    fs::create_dir_all(target_root).unwrap();

    let source_root = Path::new("temp/repositories");
    if !source_root.exists() {
        panic!("Run `examples/github_download_all_repositories.rs`")
    }

    let json = fs::read_to_string(Path::new("temp/repositories.json")).unwrap();
    let repositories: Vec<String> = serde_json::from_str(&json).unwrap();
    for full_name in repositories {
        let target_directory_name = get_crowdin_directory_name(&full_name);
        let target_directory = target_root.join(target_directory_name);
        fs::create_dir(&target_directory).unwrap();

        let (_owner, repo) = full_name.split_once('/').unwrap();
        let source_directory = source_root.join(repo).join("locale/en");

        for source_path in util::get_directory_cfg_files_paths(&source_directory) {
            let source_file_name = util::file_name(&source_path);
            let target_file_name = replace_cfg_to_ini(source_file_name);
            let target_path = target_directory.join(target_file_name);

            let content = fs::read_to_string(source_path).unwrap();
            let content = escape_strings_in_ini_file(&content);
            fs::write(target_path, content).unwrap();
        }
    }
}
