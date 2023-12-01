use fml::github;

#[tokio::main]
async fn main() {
    fml::init();

    let not_starred = github::get_not_starred_repositories().await;

    let api_personal = github::as_personal_account();
    for full_name in not_starred {
        println!("Starring {}", full_name);
        github::star_repository(&api_personal, &full_name).await;
    }
}
