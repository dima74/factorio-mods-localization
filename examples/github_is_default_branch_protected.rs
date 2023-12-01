use fml::github;

#[tokio::main]
async fn main() {
    fml::init();
    let full_name = "jingleheimer-schmidt/factorio-trainsaver";  // true
    // let full_name = "jingleheimer-schmidt/cutscene-creator";  // false
    let api = github::as_installation_for_user("jingleheimer-schmidt").await;
    let is_protected = github::is_default_branch_protected(&api, full_name).await;
    dbg!(is_protected);
}
