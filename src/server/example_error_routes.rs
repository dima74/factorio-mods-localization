/// For testing Sentry

use std::time::Duration;

use log::info;
use rocket::get;

#[get("/error1")]
pub fn error1() -> String {
    info!("info message");
    panic!("Example error 1")
}

#[get("/error2")]
pub async fn error2() -> String {
    tokio::time::sleep(Duration::from_secs(1)).await;
    panic!("Example error 2");
}
