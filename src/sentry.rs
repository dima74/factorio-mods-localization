use log::error;
use sentry::Level;
use sentry_log::LogFilter;

// https://docs.sentry.io/platforms/rust/
pub fn init_sentry() -> sentry::ClientInitGuard {
    sentry::init(sentry::ClientOptions {
        release: sentry::release_name!(),
        ..Default::default()
    })
}

// Analogue of `pretty_env_logger::init();` but with sentry middleware
// Note that it overrides rocket logging
pub fn init_logging() {
    let mut builder = pretty_env_logger::formatted_builder();
    builder.parse_default_env();
    let logger = builder.build();
    let max_level = logger.filter();

    let sentry_logger = sentry_log::SentryLogger::with_dest(logger)
        .filter(|data| match data.level() {
            // Ignore Level::Error because rocket prints error when route handler panics,
            // so we end up with duplicated events on sentry (one for panic, one for rocket error log).
            log::Level::Warn | log::Level::Info => LogFilter::Breadcrumb,
            _ => LogFilter::Ignore,
        });
    log::set_boxed_logger(Box::new(sentry_logger)).unwrap();
    log::set_max_level(max_level);
}

pub fn sentry_report_error(message: &str) {
    error!("{}", message);
    sentry::capture_message(message, Level::Error);
}
