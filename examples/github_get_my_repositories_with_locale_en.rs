use fml::github;

#[tokio::main]
async fn main() {
    fml::init();
    let api = github::as_installation_for_user("dima74").await;
    let repositories = github::get_repositories_of_installation(&api).await;
    dbg!(repositories);
}
