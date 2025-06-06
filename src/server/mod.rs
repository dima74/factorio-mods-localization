use std::ops::Deref;

use log::info;
use rocket::{get, post, routes};
use rocket::response::content::RawHtml;

use crate::myenv::WEBSERVER_SECRET;
use crate::server::webhook_util::GithubEvent;
use crate::webhooks;

mod debug_routes;
mod example_error_routes;
mod trigger_update;
mod trigger_update_public;
pub mod webhook_util;

#[get("/")]
fn index() -> RawHtml<&'static str> {
    RawHtml("<p>Factorio mods localization</p><p>See <a href='https://github.com/dima74/factorio-mods-localization'>GitHub repository</a> for documentation</p>")
}

#[post("/webhook", format = "json", data = "<event>")]
fn webhook(event: GithubEvent) {
    let task = webhooks::webhook_impl(event.0);
    // execute task in another thread, because it may be long
    tokio::spawn(task);
}

#[get("/version")]
fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

fn check_secret(secret: Option<String>) -> bool {
    secret.as_ref() == Some(WEBSERVER_SECRET.deref())
}

pub async fn main() {
    info!("launching Rocket...");
    let routes = routes![
        index,
        webhook,
        trigger_update::trigger_update,
        trigger_update_public::trigger_update,
        trigger_update_public::trigger_update2,
        version,
        debug_routes::import_repository,
        debug_routes::import_english,
        debug_routes::list_repositories,
        debug_routes::list_repositories_for_user,
        debug_routes::list_users,
        debug_routes::trigger_oom,
        example_error_routes::error1,
        example_error_routes::error2,
    ];
    rocket::build()
        .mount("/", routes)
        .launch().await.unwrap();
}
