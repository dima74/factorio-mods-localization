use std::path::Path;
use std::process::Command;

use log::{error, warn};

use crate::github::{GITHUB_BRANCH_NAME, GITHUB_USER_NAME};

pub fn clone(url: &str, path: &Path) {
    execute_git_command(
        &path,
        &[
            "clone",
            "--depth", "1",
            url,
            ".",  // clone to current directory
        ],
        true,
    )
}

pub fn add_all_and_check_has_changes(path: &Path) -> bool {
    add_all(path);
    has_changes(path)
}

fn add_all(path: &Path) {
    execute_git_command(&path, &["add", "."], true);
}

pub fn commit(path: &Path) {
    let name = dotenv::var("GIT_COMMIT_USER_NAME").unwrap();
    let email = dotenv::var("GIT_COMMIT_USER_EMAIL").unwrap();
    let message = dotenv::var("GIT_COMMIT_MESSAGE").unwrap();
    let args = &[
        "-c", &format!("user.name='{}'", name),
        "-c", &format!("user.email='{}'", email),
        "commit",
        "-m", &message
    ];
    execute_git_command(&path, args, true);
}

pub fn push(path: &Path) {
    execute_git_command(&path, &["push"], false);
}

pub fn push_to_my_fork(path: &Path, repo: &str) {
    let personal_token = dotenv::var("GITHUB_PERSONAL_ACCESS_TOKEN").unwrap();
    let url = format!("https://x-access-token:{}@github.com/{}/{}.git", personal_token, GITHUB_USER_NAME, repo);
    execute_git_command(&path, &["remote", "add", "my", &url], true);

    let refspec = format!("HEAD:{}", GITHUB_BRANCH_NAME);
    execute_git_command(&path, &["push", "my", &refspec, "--force"], true);
}

fn has_changes(path: &Path) -> bool {
    let changes = Command::new("git")
        .current_dir(path)
        .args(&["status", "--porcelain"])
        .output()
        .expect("Failed to execute git command")
        .stdout;
    !changes.is_empty()
}

fn execute_git_command(path: &Path, args: &[&str], panic_if_fail: bool) {
    let result = Command::new("git")
        .current_dir(path)
        .args(args)
        .output()
        .expect("Failed to execute git command");
    if !result.status.success() {
        let stderr = String::from_utf8_lossy(&result.stderr);
        let message = format!("Failed to execute `git {}`", args.join(" "));
        if panic_if_fail {
            error!("{}", stderr);
            panic!("{}", message);
        } else {
            warn!("{}", stderr);
            warn!("{}", message);
        }
    }
}
