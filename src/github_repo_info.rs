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
    pub mods: Vec<GithubModName>,
}

impl GithubRepoInfo {
    pub fn new_from_config(
        mods: Vec<GithubModName>,
    ) -> Option<Self> {
        if mods.is_empty() { return None; }
        Some(Self {
            mods,
        })
    }

    pub fn new_single_mod(full_name: &str) -> Self {
        let mods = vec![GithubModName::new(full_name, None, None)];
        Self {
            mods,
        }
    }

    // for debug routes
    pub fn new_one_mod_with_subpath(full_name: &str, subpath: String) -> Self {
        let mods = vec![GithubModName::new(full_name, Some(subpath), None)];
        Self {
            mods,
        }
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
    // This is not good design of data structure. Instead this should be something like this:
    // struct GithubRepoInfo { mods: Vec<...>, weekly_update_from_crowdin: bool }
    pub weekly_update_from_crowdin: bool,
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
    pub fn new(full_name: &str, subpath: Option<String>, weekly_update_from_crowdin: Option<bool>) -> Self {
        let (owner, repo) = full_name.split_once('/').unwrap();
        Self {
            owner: owner.to_owned(),
            repo: repo.to_owned(),
            subpath,
            weekly_update_from_crowdin: weekly_update_from_crowdin.unwrap_or(true),
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
    let (mods, weekly_update_from_crowdin) = match serde_json::from_str::<Data>(&json) {
        Ok(data) => {
            (data.mods, data.weekly_update_from_crowdin)
        }
        Err(_) => {
            let mods: Vec<String> = serde_json::from_str(&json).unwrap();
            (Some(mods), None)
        }
    };
    let mods = match mods {
        None => {
            // { "weekly_update_from_crowdin": false }
            vec![GithubModName::new(full_name, None, weekly_update_from_crowdin)]
        }
        Some(mods) => {
            mods
                .into_iter()
                .map(|name| {
                    // only direct subdirectories are supported
                    assert!(name != "" && !name.starts_with(".") && !name.contains('/'));
                    GithubModName::new(full_name, Some(name), weekly_update_from_crowdin)
                })
                .collect()
        }
    };
    GithubRepoInfo::new_from_config(mods)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_github_mod_names_json() {
        assert_eq!(
            parse_github_repo_info_json("owner/repo", r#"["mod1", "mod2"]"#),
            Some(GithubRepoInfo {
                mods: vec![
                    GithubModName {
                        owner: "owner".to_owned(),
                        repo: "repo".to_owned(),
                        subpath: Some("mod1".to_owned()),
                        weekly_update_from_crowdin: true,
                    },
                    GithubModName {
                        owner: "owner".to_owned(),
                        repo: "repo".to_owned(),
                        subpath: Some("mod2".to_owned()),
                        weekly_update_from_crowdin: true,
                    },
                ],
            })
        );
        assert_eq!(
            parse_github_repo_info_json("owner/repo", r#"{"mods": ["mod1", "mod2"]}"#),
            Some(GithubRepoInfo {
                mods:
                vec![
                    GithubModName {
                        owner: "owner".to_owned(),
                        repo: "repo".to_owned(),
                        subpath: Some("mod1".to_owned()),
                        weekly_update_from_crowdin: true,
                    },
                    GithubModName {
                        owner: "owner".to_owned(),
                        repo: "repo".to_owned(),
                        subpath: Some("mod2".to_owned()),
                        weekly_update_from_crowdin: true,
                    },
                ],
            })
        );
        assert_eq!(
            parse_github_repo_info_json("owner/repo", r#"{"mods": ["mod1", "mod2"], "weekly_update_from_crowdin": false}"#),
            Some(GithubRepoInfo {
                mods: vec![
                    GithubModName {
                        owner: "owner".to_owned(),
                        repo: "repo".to_owned(),
                        subpath: Some("mod1".to_owned()),
                        weekly_update_from_crowdin: false,
                    },
                    GithubModName {
                        owner: "owner".to_owned(),
                        repo: "repo".to_owned(),
                        subpath: Some("mod2".to_owned()),
                        weekly_update_from_crowdin: false,
                    },
                ],
            })
        );
        assert_eq!(
            parse_github_repo_info_json("owner/repo", r#"{"weekly_update_from_crowdin": false}"#),
            Some(GithubRepoInfo {
                mods: vec![
                    GithubModName {
                        owner: "owner".to_owned(),
                        repo: "repo".to_owned(),
                        subpath: None,
                        weekly_update_from_crowdin: false,
                    },
                ],
            })
        );
        assert_eq!(
            parse_github_repo_info_json("owner/repo", r#"{"weekly_update_from_crowdin": true}"#),
            Some(GithubRepoInfo {
                mods: vec![
                    GithubModName {
                        owner: "owner".to_owned(),
                        repo: "repo".to_owned(),
                        subpath: None,
                        weekly_update_from_crowdin: true,
                    },
                ],
            })
        );
    }
}
