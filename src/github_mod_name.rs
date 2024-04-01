use std::fmt;

use serde::Deserialize;

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
/// Old format:
/// ```json
/// ["mod1", "mod2"]
/// ```
///
/// New format:
/// ```json
/// {
///   "mods": ["mod1", "mod2"],
///   "weekly_update_from_crowdin": false
/// }
/// ```
pub fn parse_github_mod_names_json(full_name: &str, json: &str) -> Vec<GithubModName> {
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
    match mods {
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_github_mod_names_json() {
        assert_eq!(
            parse_github_mod_names_json("owner/repo", r#"["mod1", "mod2"]"#),
            vec![
                GithubModName {
                    owner: "owner".to_owned(),
                    repo: "repo".to_owned(),
                    subpath: Some("mod1".to_owned()),
                    weekly_update_from_crowdin: true
                },
                GithubModName {
                    owner: "owner".to_owned(),
                    repo: "repo".to_owned(),
                    subpath: Some("mod2".to_owned()),
                    weekly_update_from_crowdin: true
                },
            ]
        );
        assert_eq!(
            parse_github_mod_names_json("owner/repo", r#"{"mods": ["mod1", "mod2"]}"#),
            vec![
                GithubModName {
                    owner: "owner".to_owned(),
                    repo: "repo".to_owned(),
                    subpath: Some("mod1".to_owned()),
                    weekly_update_from_crowdin: true
                },
                GithubModName {
                    owner: "owner".to_owned(),
                    repo: "repo".to_owned(),
                    subpath: Some("mod2".to_owned()),
                    weekly_update_from_crowdin: true
                },
            ]
        );
        assert_eq!(
            parse_github_mod_names_json("owner/repo", r#"{"mods": ["mod1", "mod2"], "weekly_update_from_crowdin": false}"#),
            vec![
                GithubModName {
                    owner: "owner".to_owned(),
                    repo: "repo".to_owned(),
                    subpath: Some("mod1".to_owned()),
                    weekly_update_from_crowdin: false
                },
                GithubModName {
                    owner: "owner".to_owned(),
                    repo: "repo".to_owned(),
                    subpath: Some("mod2".to_owned()),
                    weekly_update_from_crowdin: false
                },
            ]
        );
        assert_eq!(
            parse_github_mod_names_json("owner/repo", r#"{"weekly_update_from_crowdin": false}"#),
            vec![
                GithubModName {
                    owner: "owner".to_owned(),
                    repo: "repo".to_owned(),
                    subpath: None,
                    weekly_update_from_crowdin: false
                },
            ]
        );
        assert_eq!(
            parse_github_mod_names_json("owner/repo", r#"{"weekly_update_from_crowdin": true}"#),
            vec![
                GithubModName {
                    owner: "owner".to_owned(),
                    repo: "repo".to_owned(),
                    subpath: None,
                    weekly_update_from_crowdin: true
                },
            ]
        );
    }
}
