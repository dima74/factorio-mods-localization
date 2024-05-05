use std::collections::HashSet;
use std::fmt;

use serde::Deserialize;

use crate::crowdin::get_crowdin_directory_name;

/// `factorio-mods-localization.json` - config file in root of the repository
///
/// ```json
/// {
///   "mods": ["mod1", "mod2"],
///   "weekly_update_from_crowdin": false,
/// }
/// ```
#[derive(Debug, Eq, PartialEq)]
pub struct GithubRepoInfo {
    pub full_name: String,
    pub mods: Vec<GithubModName>,
    pub weekly_update_from_crowdin: bool,
}

impl GithubRepoInfo {
    fn new(
        full_name: &str,
        mods: Vec<GithubModName>,
        weekly_update_from_crowdin: Option<bool>,
    ) -> Self {
        Self {
            full_name: full_name.to_owned(),
            mods,
            weekly_update_from_crowdin: weekly_update_from_crowdin.unwrap_or(true),
        }
    }

    pub fn new_from_config(
        full_name: &str,
        mods: Vec<GithubModName>,
        weekly_update_from_crowdin: Option<bool>,
    ) -> Option<Self> {
        if mods.is_empty() { return None; }
        Some(Self::new(full_name, mods, weekly_update_from_crowdin))
    }

    pub fn new_single_mod(full_name: &str) -> Self {
        let mods = vec![GithubModName::new(full_name, None)];
        Self::new(full_name, mods, None)
    }

    // for debug routes
    pub fn new_one_mod_with_subpath(full_name: &str, subpath: String) -> Self {
        let mods = vec![GithubModName::new(full_name, Some(subpath))];
        Self::new(full_name, mods, None)
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

/// # Single mod in github repository
/// .
/// ├── locale/en
///
/// # Multiple mods in github repository
/// .
/// ├── factorio-mods-localization.json  // {"mods": ["Mod1", "Mod2"]}
/// ├── Mod1
/// │   ├── locale/en
/// ├── Mod2
/// │   ├── locale/en
///
#[derive(Debug, Eq, PartialEq)]
pub struct GithubModName {
    pub owner: String,
    pub repo: String,
    pub subpath: Option<String>,
}

impl fmt::Display for GithubModName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.subpath {
            None => write!(f, "{}/{}", self.owner, self.repo),
            Some(subpath) => write!(f, "{}/{}/{}", self.owner, self.repo, subpath),
        }
    }
}

impl GithubModName {
    pub fn new(full_name: &str, subpath: Option<String>) -> Self {
        let (owner, repo) = full_name.split_once('/').unwrap();
        Self {
            owner: owner.to_owned(),
            repo: repo.to_owned(),
            subpath,
        }
    }
}

/// # Format of factorio-mods-localization.json
/// Old format (deprecated):
/// ```json
/// ["mod1", "mod2"]
/// ```
///
/// New format:
/// ```json
/// {
///   "mods": ["mod1", "mod2"],
///   ...
/// }
/// ```
pub fn parse_github_repo_info_json(full_name: &str, json: &str) -> Option<GithubRepoInfo> {
    #[derive(Deserialize)]
    struct Data {
        mods: Option<Vec<String>>,
        weekly_update_from_crowdin: Option<bool>,
    }
    let data = serde_json::from_str::<Data>(&json)
        .unwrap_or_else(|_| {
            let mods = serde_json::from_str(&json).unwrap();
            Data {
                mods: Some(mods),
                weekly_update_from_crowdin: None,
            }
        });
    let mods = match data.mods {
        None => {
            // { "weekly_update_from_crowdin": false }
            vec![GithubModName::new(full_name, None)]
        }
        Some(mods) => {
            mods
                .into_iter()
                .map(|name| {
                    // only direct subdirectories are supported
                    assert!(name != "" && !name.starts_with(".") && !name.contains('/'));
                    GithubModName::new(full_name, Some(name))
                })
                .collect()
        }
    };
    GithubRepoInfo::new_from_config(full_name, mods, data.weekly_update_from_crowdin)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_github_mod_names_json() {
        assert_eq!(
            parse_github_repo_info_json("owner/repo", r#"["mod1", "mod2"]"#),
            Some(GithubRepoInfo {
                full_name: "owner/repo".to_owned(),
                mods: vec![
                    GithubModName {
                        owner: "owner".to_owned(),
                        repo: "repo".to_owned(),
                        subpath: Some("mod1".to_owned()),
                    },
                    GithubModName {
                        owner: "owner".to_owned(),
                        repo: "repo".to_owned(),
                        subpath: Some("mod2".to_owned()),
                    },
                ],
                weekly_update_from_crowdin: true,
            })
        );
        assert_eq!(
            parse_github_repo_info_json("owner/repo", r#"{"mods": ["mod1", "mod2"]}"#),
            Some(GithubRepoInfo {
                full_name: "owner/repo".to_owned(),
                mods:
                vec![
                    GithubModName {
                        owner: "owner".to_owned(),
                        repo: "repo".to_owned(),
                        subpath: Some("mod1".to_owned()),
                    },
                    GithubModName {
                        owner: "owner".to_owned(),
                        repo: "repo".to_owned(),
                        subpath: Some("mod2".to_owned()),
                    },
                ],
                weekly_update_from_crowdin: true,
            })
        );
        assert_eq!(
            parse_github_repo_info_json("owner/repo", r#"{"mods": ["mod1", "mod2"], "weekly_update_from_crowdin": false}"#),
            Some(GithubRepoInfo {
                full_name: "owner/repo".to_owned(),
                mods: vec![
                    GithubModName {
                        owner: "owner".to_owned(),
                        repo: "repo".to_owned(),
                        subpath: Some("mod1".to_owned()),
                    },
                    GithubModName {
                        owner: "owner".to_owned(),
                        repo: "repo".to_owned(),
                        subpath: Some("mod2".to_owned()),
                    },
                ],
                weekly_update_from_crowdin: false,
            })
        );
        assert_eq!(
            parse_github_repo_info_json("owner/repo", r#"{"weekly_update_from_crowdin": false}"#),
            Some(GithubRepoInfo {
                full_name: "owner/repo".to_owned(),
                mods: vec![
                    GithubModName {
                        owner: "owner".to_owned(),
                        repo: "repo".to_owned(),
                        subpath: None,
                    },
                ],
                weekly_update_from_crowdin: false,
            })
        );
        assert_eq!(
            parse_github_repo_info_json("owner/repo", r#"{"weekly_update_from_crowdin": true}"#),
            Some(GithubRepoInfo {
                full_name: "owner/repo".to_owned(),
                mods: vec![
                    GithubModName {
                        owner: "owner".to_owned(),
                        repo: "repo".to_owned(),
                        subpath: None,
                    },
                ],
                weekly_update_from_crowdin: true,
            })
        );
    }
}
