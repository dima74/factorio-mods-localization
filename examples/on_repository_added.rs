use fml::{github, webhooks};
use fml::github_mod_name::GithubModName;

#[tokio::main]
async fn main() {
    fml::init_with_crowdin().await;
    let installation_id = github::get_installation_id_for_user("dima74").await;
    let mod_ = GithubModName::new("dima74/factorio-mod-example", None, None);
    webhooks::on_repository_added("dima74/factorio-mod-example", vec![mod_], installation_id).await;
}
