//! General overview of the process:
//! 1. GitHub app installed - [webhooks::on_repositories_added]
//! 2. English files updated on GitHub - [webhooks::on_push_event]
//! 3. Weekly update from Crowdin to GitHub - [server::trigger_update::push_all_crowdin_changes_to_github]

pub mod crowdin;
pub mod git_util;
pub mod github;
pub mod mod_directory;
pub mod myenv;
pub mod sentry;
pub mod server;
pub mod util;
pub mod webhooks;
pub mod github_repo_info;
pub mod github_config;

pub fn init() {
    dotenv::dotenv().ok();
    myenv::init();
    sentry::init_logging();
}

pub async fn init_with_crowdin() {
    init();
    crowdin::init().await;
}

pub async fn main() {
    init_with_crowdin().await;
    server::main().await;
}
