#[tokio::main]
async fn main() {
    fml::init_with_crowdin().await;
    let path = fml::crowdin::download_all_translations().await;
    dbg!(&path);
    #[allow(clippy::empty_loop)]
    loop {}
}
