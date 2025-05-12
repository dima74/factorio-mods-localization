//! `factorio-mods-localization.json` - config file in root of the repository.
//! It should be in the *default* branch, even if some other "branch" is specified in config.
//!
//! # Format of `factorio-mods-localization.json`
//! Old format (deprecated):
//! ```json
//! ["mod1", "mod2"]
//! ```
//!
//! New format:
//! ```json
//! {
//!   "mods": ["mod1", "mod2"],
//!   "weekly_update_from_crowdin": false,
//!   "branch": "dev"
//! }
//! ```
//!
//! Alternative format for "mods":
//! ```json
//! {
//!   "mods": [{"localePath": "custom/path", "crowdinName": "Foo"}]
//!   ...
//! }
//! ```
//!
//! # Examples
//!
//! ## Single mod in github repository (no `factorio-mods-localization.json`)
//! .
//! ├── locale/en
//!
//! ## Multiple mods in github repository
//! .
//! ├── factorio-mods-localization.json  // `{"mods": ["Mod1", "Mod2"]}`
//! ├── Mod1
//! │   ├── locale/en
//! ├── Mod2
//! │   ├── locale/en

use crate::github_repo_info::{GithubModInfo, GithubRepoInfo};
use serde::Deserialize;
use std::collections::HashSet;

#[derive(Deserialize)]
struct Config {
    mods: Option<ConfigMods>,
    weekly_update_from_crowdin: Option<bool>,
    branch: Option<String>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum ConfigMods {
    Short(Vec<String>),
    Full(Vec<ConfigMod>),
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ConfigMod {
    locale_path: String,
    crowdin_name: String,
}

#[derive(Deserialize)]
struct ConfigOld(Vec<String>);

impl From<ConfigOld> for Config {
    fn from(config: ConfigOld) -> Self {
        Config {
            mods: Some(ConfigMods::Short(config.0)),
            weekly_update_from_crowdin: None,
            branch: None,
        }
    }
}

pub fn parse_github_repo_info_json(full_name: &str, json: &str) -> Option<GithubRepoInfo> {
    let config: Config = parse_config(json)?;
    let mods = convert_mods(full_name, config.mods)?;
    if !check_no_duplicates(&mods) { return None; }
    GithubRepoInfo::new_from_config(full_name, mods, config.weekly_update_from_crowdin, config.branch)
}

fn parse_config(json: &str) -> Option<Config> {
    if let Ok(config) = serde_json::from_str::<ConfigOld>(json) {
        return Some(config.into());
    }
    serde_json::from_str(json).ok()
}

fn convert_mods(
    full_name: &str,
    mods: Option<ConfigMods>,
) -> Option<Vec<GithubModInfo>> {
    let Some(mods) = mods else {
        // { "weekly_update_from_crowdin": false }
        return Some(vec![GithubModInfo::new_root(full_name)]);
    };

    match mods {
        ConfigMods::Short(mods) => {
            mods
                .into_iter()
                .map(|name| GithubModInfo::new_custom(full_name, None, name))
                .collect()
        }
        ConfigMods::Full(mods) => {
            mods
                .into_iter()
                .map(|mod_| {
                    GithubModInfo::new_custom(full_name, Some(mod_.locale_path), mod_.crowdin_name)
                })
                .collect()
        }
    }
}

fn check_no_duplicates(mods: &[GithubModInfo]) -> bool {
    let mods_set = mods
        .iter()
        .map(|mod_| mod_.crowdin_name.as_ref())
        .collect::<HashSet<_>>();
    mods.len() == mods_set.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mods_old_version() {
        assert_eq!(
            parse_github_repo_info_json("owner/repo", r#"["mod1", "mod2"]"#),
            Some(GithubRepoInfo {
                full_name: "owner/repo".to_owned(),
                mods: vec![
                    GithubModInfo {
                        owner: "owner".to_owned(),
                        repo: "repo".to_owned(),
                        locale_path: "mod1/locale".to_owned(),
                        crowdin_name: Some("mod1".to_owned()),
                    },
                    GithubModInfo {
                        owner: "owner".to_owned(),
                        repo: "repo".to_owned(),
                        locale_path: "mod2/locale".to_owned(),
                        crowdin_name: Some("mod2".to_owned()),
                    },
                ],
                weekly_update_from_crowdin: true,
                branch: None,
            })
        );
    }
    
    #[test]
    fn test_parse_mods_short_version() {
        assert_eq!(
            parse_github_repo_info_json("owner/repo", r#"{"mods": ["mod1", "mod2"]}"#),
            Some(GithubRepoInfo {
                full_name: "owner/repo".to_owned(),
                mods:
                vec![
                    GithubModInfo {
                        owner: "owner".to_owned(),
                        repo: "repo".to_owned(),
                        locale_path: "mod1/locale".to_owned(),
                        crowdin_name: Some("mod1".to_owned()),
                    },
                    GithubModInfo {
                        owner: "owner".to_owned(),
                        repo: "repo".to_owned(),
                        locale_path: "mod2/locale".to_owned(),
                        crowdin_name: Some("mod2".to_owned()),
                    },
                ],
                weekly_update_from_crowdin: true,
                branch: None,
            })
        );
    }

    #[test]
    fn test_parse_mods_long_version() {
        assert_eq!(
            parse_github_repo_info_json("owner/repo", r#"{"mods": [{"localePath": "custom/path", "crowdinName": "Foo"}]}"#),
            Some(GithubRepoInfo {
                full_name: "owner/repo".to_owned(),
                mods:
                vec![
                    GithubModInfo {
                        owner: "owner".to_owned(),
                        repo: "repo".to_owned(),
                        locale_path: "custom/path".to_owned(),
                        crowdin_name: Some("Foo".to_owned()),
                    },
                ],
                weekly_update_from_crowdin: true,
                branch: None,
            })
        )
    }

    #[test]
    fn test_parse_weekly_update_from_crowdin() {
        assert_eq!(
            parse_github_repo_info_json("owner/repo", r#"{"weekly_update_from_crowdin": false}"#),
            Some(GithubRepoInfo {
                full_name: "owner/repo".to_owned(),
                mods: vec![
                    GithubModInfo {
                        owner: "owner".to_owned(),
                        repo: "repo".to_owned(),
                        locale_path: "locale".to_owned(),
                        crowdin_name: None,
                    },
                ],
                weekly_update_from_crowdin: false,
                branch: None,
            })
        );
        assert_eq!(
            parse_github_repo_info_json("owner/repo", r#"{"weekly_update_from_crowdin": true}"#),
            Some(GithubRepoInfo {
                full_name: "owner/repo".to_owned(),
                mods: vec![
                    GithubModInfo {
                        owner: "owner".to_owned(),
                        repo: "repo".to_owned(),
                        locale_path: "locale".to_owned(),
                        crowdin_name: None,
                    },
                ],
                weekly_update_from_crowdin: true,
                branch: None,
            })
        );
    }

    #[test]
    fn test_parse_branch() {
        assert_eq!(
            parse_github_repo_info_json("owner/repo", r#"{"branch": "dev"}"#),
            Some(GithubRepoInfo {
                full_name: "owner/repo".to_owned(),
                mods: vec![
                    GithubModInfo {
                        owner: "owner".to_owned(),
                        repo: "repo".to_owned(),
                        locale_path: "locale".to_owned(),
                        crowdin_name: None,
                    },
                ],
                weekly_update_from_crowdin: true,
                branch: Some("dev".to_owned()),
            })
        );
    }

    #[test]
    fn test_parse_all() {
        let json = r#"
        {
            "mods": ["mod1", "mod2"],
            "weekly_update_from_crowdin": false,
            "branch": "dev"
        }
        "#;
        assert_eq!(
            parse_github_repo_info_json("owner/repo", json),
            Some(GithubRepoInfo {
                full_name: "owner/repo".to_owned(),
                mods: vec![
                    GithubModInfo {
                        owner: "owner".to_owned(),
                        repo: "repo".to_owned(),
                        locale_path: "mod1/locale".to_owned(),
                        crowdin_name: Some("mod1".to_owned()),
                    },
                    GithubModInfo {
                        owner: "owner".to_owned(),
                        repo: "repo".to_owned(),
                        locale_path: "mod2/locale".to_owned(),
                        crowdin_name: Some("mod2".to_owned()),
                    },
                ],
                weekly_update_from_crowdin: false,
                branch: Some("dev".to_owned()),
            })
        );
    }
}
