//! Without forking contributions will not be displayed for
//! https://github.com/factorio-mods-helper

use fml::github;

#[tokio::test]
async fn main() {
    fml::init();
    let result = github::get_not_forked_repositories().await;

    let forked_with_diferrent_name = result.forked_with_diferrent_name;
    if !forked_with_diferrent_name.is_empty() {
        for full_name in &forked_with_diferrent_name {
            println!("{}", full_name);
        }
        panic!("{} repositories have forks with different name", forked_with_diferrent_name.len());
    }

    let not_forked = result.not_forked;
    if !not_forked.is_empty() {
        for full_name in &not_forked {
            println!("{}", full_name);
        }
        panic!("{} repositories not forked", not_forked.len());
    }
}
