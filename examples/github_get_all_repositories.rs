use fml::github::get_all_repositories;

#[tokio::main]
async fn main() {
    fml::init();
    let api = fml::github::as_app();
    let repositories = get_all_repositories(&api).await;
    dbg!(repositories);
}
