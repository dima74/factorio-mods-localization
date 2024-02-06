use log::info;
use rocket::{get, post, routes};
use rocket::response::content::RawHtml;

use crate::server::webhook_util::GithubEvent;
use crate::webhooks;

mod debug_routes;
mod example_error_routes;
mod trigger_update;
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

pub async fn main() {
    info!("launching Rocket...");
    let routes = routes![
        index,
        webhook,
        trigger_update::trigger_update,
        version,
        debug_routes::import_repository,
        debug_routes::import_english,
        example_error_routes::error1,
        example_error_routes::error2,
    ];
    rocket::build()
        .mount("/", routes)
        .launch().await.unwrap();
}
