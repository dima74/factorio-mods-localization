use fml::github_repo_info::GithubRepoInfo;

#[tokio::main]
async fn main() {
    fml::init();
    let api = fml::github::as_app();
    let owner = "dima74";
    let repo = "factorio-mod-example";
    let installation = api
        .apps().get_repository_installation(owner, repo)
        .await.unwrap();

    let repo_info = GithubRepoInfo::new_single_mod(&format!("{}/{}", owner, repo));
    fml::github::clone_repository(&repo_info, installation.id).await;
}
