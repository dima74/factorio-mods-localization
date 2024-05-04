use fml::{github, webhooks};
use fml::github_repo_info::GithubRepoInfo;

#[tokio::main]
async fn main() {
    fml::init_with_crowdin().await;
    let installation_id = github::get_installation_id_for_user("dima74").await;
    let repo_info = GithubRepoInfo::new_single_mod("dima74/factorio-mod-example");
    webhooks::on_repository_added("dima74/factorio-mod-example", repo_info, installation_id).await;
}
