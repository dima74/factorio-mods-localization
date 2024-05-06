use fml::github;

#[tokio::main]
async fn main() {
    fml::init();
    let full_name = "jingleheimer-schmidt/factorio-trainsaver";  // true
    // let full_name = "jingleheimer-schmidt/cutscene-creator";  // false
    let api = github::as_installation_for_user("jingleheimer-schmidt").await.unwrap();
    let default_branch = github::get_default_branch(&api, full_name).await;
    let is_protected = github::is_branch_protected(&api, full_name, &default_branch).await;
    dbg!(is_protected);
}
