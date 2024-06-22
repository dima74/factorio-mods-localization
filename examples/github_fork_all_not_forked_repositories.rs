use fml::github;

#[tokio::main]
async fn main() {
    fml::init();

    let not_forked = github::get_not_forked_repositories().await.not_forked;

    let api_personal = github::as_personal_account();
    for full_name in not_forked {
        println!("Forking {}", full_name);
        github::fork_repository_without_check(&api_personal, &full_name).await;
    }
}
