#![feature(async_fn_in_trait)]
#![feature(lazy_cell)]

pub mod crowdin;
pub mod git_util;
pub mod github;
pub mod mod_directory;
pub mod sentry;
pub mod server;
pub mod util;
pub mod webhooks;

pub fn init() {
    dotenv::dotenv().ok();
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
