use std::fs::File;
use std::io;
use std::io::{Seek, SeekFrom};
use std::path::Path;

use fml::crowdin;

#[tokio::main]
async fn main() {
    fml::init_with_crowdin().await;

    let directory_id = crowdin::find_directory_id("Factorio Mod Example (dima74)").await.unwrap();
    let mut files = crowdin::list_files(directory_id).await;
    let (_, file_id) = files.next().unwrap();
    let mut file = crowdin::download_file(file_id).await;
    file.seek(SeekFrom::Start(0)).unwrap();
    let target = Path::new("temp/download.ini");
    let mut target = File::create(target).unwrap();
    io::copy(&mut file, &mut target).unwrap();
}
