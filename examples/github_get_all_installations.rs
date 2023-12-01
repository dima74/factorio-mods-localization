// https://github.com/XAMPPRocky/octocrab/blob/9f8a94e70e4707fd240c65dd96f934d1dd46938c/examples/github_app_authentication.rs

use fml::github::get_all_installations;

#[tokio::main]
async fn main() {
    fml::init();
    let api = fml::github::as_app();
    let installations = get_all_installations(&api).await;
    dbg!(installations.len());

    let installations = installations.iter()
        .map(|it| (&it.account.login, it.id.0))
        .collect::<Vec<_>>();
    dbg!(installations);
}
