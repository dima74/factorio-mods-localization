#[tokio::main]
async fn main() {
    fml::init();
    let api = fml::github::as_app();
    let owner = "dima74";
    let repo = "factorio-mod-example";
    let installation = api
        .apps().get_repository_installation(owner, repo)
        .await.unwrap();

    fml::github::clone_repository(&format!("{}/{}", owner, repo), installation.id).await;
}
