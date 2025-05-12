//! Starring previously was needed for displaying contributions.
//! However it is not working anymore (forking is needed instead).
//! So we keep star check just so.

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
