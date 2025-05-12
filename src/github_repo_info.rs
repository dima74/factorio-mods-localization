use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;
use std::sync::LazyLock;

use crate::crowdin::get_crowdin_directory_name;

/// One [`GithubRepoInfo`] can contain multiple [`GithubModInfo`].
/// [`GithubRepoInfo`] corresponds 1-1 to github repository.
/// [`GithubModInfo`] corresponds 1-1 to directory on crowdin.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GithubRepoInfo {
    pub full_name: String,
    pub mods: Vec<GithubModInfo>,
    pub weekly_update_from_crowdin: bool,
    /// Branch from which english files will be tracked and to which translations will be pushed
    pub branch: Option<String>,
}

impl GithubRepoInfo {
    fn new(
        full_name: &str,
        mods: Vec<GithubModInfo>,
        weekly_update_from_crowdin: Option<bool>,
        branch: Option<String>,
    ) -> Self {
        Self {
            full_name: full_name.to_owned(),
            mods,
            weekly_update_from_crowdin: weekly_update_from_crowdin.unwrap_or(true),
            branch,
        }
    }

    pub fn new_from_config(
        full_name: &str,
        mods: Vec<GithubModInfo>,
        weekly_update_from_crowdin: Option<bool>,
        branch: Option<String>,
    ) -> Option<Self> {
        if mods.is_empty() { return None; }
        Some(Self::new(full_name, mods, weekly_update_from_crowdin, branch))
    }

    pub fn new_single_mod(full_name: &str) -> Self {
        let mods = vec![GithubModInfo::new_root(full_name)];
        Self::new(full_name, mods, None, None)
    }

    // for debug routes
    pub fn keep_single_mod_with_crowdin_name(&mut self, crowdin_name: &str) -> bool {
        self.mods.retain(|mod_| {
            mod_.crowdin_name.as_deref() == Some(crowdin_name)
        });
        !self.mods.is_empty()
    }

    pub fn filter_mods_present_on_crowdin(
        mut self,
        directories_crowdin: &HashSet<String>,
    ) -> Option<Self> {
        self.mods.retain(|it| directories_crowdin.contains(&get_crowdin_directory_name(it)));
        if self.mods.is_empty() { return None; }
        Some(self)
    }
}

/// Depends on `factorio-mods-localization.json`:
/// - No:
///     locale_path = "locale"
///     crowdin_name = None
/// - `["Mod1", "Mod2"]` or `{"mods": ["Mod1", "Mod2"]}`:
///     locale_path = "Mod1/locale"
///     crowdin_name = Some("Mod1")
/// - `{"mods": [{"localePath": "custom/path", "crowdinName": "Foo"}]}`
///     locale_path = "custom/path"
///     crowdin_name = Some("Foo")
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GithubModInfo {
    pub owner: String,
    pub repo: String,
    pub locale_path: String,
    pub crowdin_name: Option<String>,
}

// Used only for logging
impl fmt::Display for GithubModInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.crowdin_name {
            None => {
                write!(f, "{}/{}", self.owner, self.repo)
            }
            Some(crowdin_name) => {
                write!(f, "{}/{}/{}", self.owner, self.repo, crowdin_name)
            }
        }
    }
}

impl GithubModInfo {
    pub fn new_root(full_name: &str) -> Self {
        Self::new(full_name, "locale".to_owned(), None)
    }

    pub fn new_custom(
        full_name: &str,
        locale_path: Option<String>,
        crowdin_name: String,
    ) -> Option<Self> {
        let locale_path = locale_path
            .unwrap_or_else(|| format!("{crowdin_name}/locale"));

        if !Self::check_locale_path(&locale_path) { return None; }
        if !Self::check_crowdin_name(&crowdin_name) { return None; }

        Some(Self::new(full_name, locale_path, Some(crowdin_name)))
    }

    fn new(
        full_name: &str,
        locale_path: String,
        crowdin_name: Option<String>,
    ) -> Self {
        let (owner, repo) = full_name.split_once('/').unwrap();
        Self {
            owner: owner.to_owned(),
            repo: repo.to_owned(),
            locale_path,
            crowdin_name,
        }
    }

    fn check_locale_path(locale_path: &str) -> bool {
        !locale_path.is_empty()
            && !locale_path.contains(['.', ' ', '<', '>', ':', '"', '\\', '|', '?', '*'])
            && !locale_path.split('/').any(|s| s.is_empty())
    }

    fn check_crowdin_name(crowdin_name: &str) -> bool {
        static CROWDIN_NAME_REGEX: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"^[a-zA-Z0-9._-]+$").unwrap()
        });
        CROWDIN_NAME_REGEX.is_match(crowdin_name)
    }
}
