use std::ops::Deref;
use std::sync::LazyLock;

#[derive(Eq, PartialEq)]
enum Environment {
    /// Env not checked
    DEVELOPMENT,
    /// Some env are required
    CI,
    /// All env are required
    PRODUCTION,
}

fn current_environment() -> Environment {
    if is_development() {
        return Environment::DEVELOPMENT;
    }
    if dotenv::var("CI").is_ok() {
        return Environment::CI;
    }
    return Environment::PRODUCTION;
}

pub fn is_development() -> bool {
    dotenv::var("IS_DEVELOPMENT").ok() == Some("true".to_owned())
}

macro_rules! gen {
    ($($ci:literal $name:ident),* $(,)?) => {
        $(
            pub static $name: LazyLock<String> = LazyLock::new(|| dotenv::var(stringify!($name)).unwrap());
        )*
        pub fn init() {
            let environment = current_environment();
            if environment == Environment::DEVELOPMENT { return; }
            $(
                if $ci == 1 || environment == Environment::PRODUCTION {
                    let _ = $name.deref();
                }
            )*
        }
    }
}

// 1 if env is needed both for CI and production
// 0 if env is needed only for production
gen!(
    1 CROWDIN_PROJECT_ID,
    1 CROWDIN_API_KEY,
    1 GITHUB_APP_ID,
    1 GITHUB_APP_PRIVATE_KEY,
    0 GITHUB_APP_WEBHOOKS_SECRET,
    1 GITHUB_PERSONAL_ACCESS_TOKEN,
    0 GITHUB_OAUTH_CLIENT_ID,
    0 GITHUB_OAUTH_CLIENT_SECRET,
    0 SENTRY_DSN,
    0 GIT_COMMIT_USER_NAME,
    0 GIT_COMMIT_USER_EMAIL,
    0 GIT_COMMIT_MESSAGE,
    0 WEBSERVER_SECRET,
);
