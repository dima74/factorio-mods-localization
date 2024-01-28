use std::fmt;

/// # Single mod in github repository
/// .
/// ├── locale/en
///
/// # Multiple mods in github repository
/// .
/// ├── factorio-mods-localization.json  // ["Mod1", "Mod2"]
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

pub fn parse_github_mod_names_json(full_name: &str, json: &str) -> Vec<GithubModName> {
    let names: Vec<String> = serde_json::from_str(&json).unwrap();
    names
        .into_iter()
        .map(|name| {
            // only direct subdirectories are supported
            assert!(name != "" && !name.starts_with(".") && !name.contains('/'));
            GithubModName::new(full_name, Some(name))
        })
        .collect()
}
