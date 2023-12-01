use fml::github;

#[tokio::test]
async fn main() {
    fml::init();
    let not_starred = github::get_not_starred_repositories().await;
    if !not_starred.is_empty() {
        for full_name in &not_starred {
            println!("{}", full_name);
        }
        panic!("{} repositories not starred", not_starred.len());
    }
}
