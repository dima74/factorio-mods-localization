fn main() {
    dotenv::dotenv().ok();
    let _sentry = fml::sentry::init_sentry();
    rocket::async_main(fml::main());
}
