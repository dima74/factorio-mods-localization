use fml::{github, webhooks};

#[tokio::main]
async fn main() {
    fml::init_with_crowdin().await;
    let installation_id = github::get_installation_id_for_user("dima74").await;
    webhooks::on_repository_added("dima74/factorio-mod-example", installation_id).await;
}
