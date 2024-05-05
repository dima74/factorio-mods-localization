//! Comparison with [super::trigger_update]:
//! `/triggerUpdate?secret=X&repo=REPO` - for private use, secret based authorization
//! `/api/triggerUpdate?repo=REPO` - for public use, GitHub OAuth based authorization

use std::ops::Deref;

use rocket::get;
use rocket::response::Redirect;
use url::Url;

use crate::github;
use crate::myenv::GITHUB_OAUTH_CLIENT_ID;
use crate::server::trigger_update::trigger_update_single_repository_part1;

#[get("/api/triggerUpdate?<repo>")]
pub async fn trigger_update(repo: Option<String>) -> Result<Redirect, &'static str> {
    let Some(repo) = repo else {
        return Err("Missing `repo` query parameter");
    };
    if repo.chars().filter(|it| *it == '/').count() != 1
        || repo.starts_with('/') || repo.ends_with('/') {
        return Err("`repo` parameter should be in format `owner/repo`");
    }
    let owner = repo.split_once('/').unwrap().0;

    let mut redirect_url = Url::parse("https://factorio-mods-localization.fly.dev/api/triggerUpdate2").unwrap();
    redirect_url.query_pairs_mut()
        .append_pair("repo", &repo);

    let mut url = Url::parse("https://github.com/login/oauth/authorize").unwrap();
    url.query_pairs_mut()
        .append_pair("client_id", GITHUB_OAUTH_CLIENT_ID.deref())
        .append_pair("redirect_uri", redirect_url.as_ref())
        .append_pair("login", owner)
        .append_pair("allow_signup", "false");
    Ok(Redirect::to(url.to_string()))
}

#[get("/api/triggerUpdate2?<repo>&<code>")]
pub async fn trigger_update2(repo: String, code: String) -> String {
    let owner = repo.split_once('/').unwrap().0;

    let api = github_oauth::authenticate(&code).await;
    let authenticated_user = github::get_current_user(&api).await;

    if owner != authenticated_user {
        return format!("Authentication failed, expected `{}` user, found `{}`", owner, authenticated_user);
    }

    match trigger_update_single_repository_part1(repo, None).await {
        Ok(_) => "Triggered. Repository will be updated if there are changes on Crowdin.".to_owned(),
        Err(e) => e.to_owned(),
    }
}

// this should be in octocrab library
mod github_oauth {
    use std::collections::HashMap;
    use std::ops::Deref;

    use http::header::ACCEPT;
    use octocrab::auth::OAuth;
    use octocrab::Octocrab;
    use rocket::serde::Deserialize;
    use secrecy::SecretString;

    use crate::myenv::{GITHUB_OAUTH_CLIENT_ID, GITHUB_OAUTH_CLIENT_SECRET};

    pub async fn authenticate(code: &str) -> Octocrab {
        let oauth = get_access_token(code).await;
        Octocrab::builder().oauth(oauth).build().unwrap()
    }

    async fn get_access_token(code: &str) -> OAuth {
        let mut params = HashMap::<&str, &str>::new();
        params.insert("client_id", GITHUB_OAUTH_CLIENT_ID.deref());
        params.insert("client_secret", GITHUB_OAUTH_CLIENT_SECRET.deref());
        params.insert("code", code);
        let api = Octocrab::builder()
            .add_header(ACCEPT, "application/json".to_string())
            .build()
            .unwrap();
        let response: OAuthWire = api
            .post("https://github.com/login/oauth/access_token", Some(&params))
            .await
            .unwrap();
        response.into()
    }

    #[derive(Deserialize)]
    struct OAuthWire {
        access_token: String,
        token_type: String,
        scope: String,
    }

    impl From<OAuthWire> for OAuth {
        fn from(value: OAuthWire) -> Self {
            OAuth {
                access_token: SecretString::from(value.access_token),
                token_type: value.token_type,
                scope: value.scope.split(',').map(ToString::to_string).collect(),
            }
        }
    }
}
