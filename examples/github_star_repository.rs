use fml::github;

#[tokio::main]
async fn main() {
    fml::init();
    let api_personal = github::as_personal_account();
    github::star_repository(&api_personal, "dima74/factorio-mod-example").await;
}
