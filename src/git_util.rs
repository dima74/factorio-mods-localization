use std::path::Path;
use std::process::{Command, Output};

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
    git_status_has_changes(path)
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
        "-c", "commit.gpgsign=false",
        "commit",
        "-m", &message
    ];
    execute_git_command(&path, args, true);
}

pub fn push(path: &Path) {
    execute_git_command(&path, &["push"], false);
}

pub fn push_to_my_fork(path: &Path, repo: &str) -> bool {
    let personal_token = dotenv::var("GITHUB_PERSONAL_ACCESS_TOKEN").unwrap();
    let url = format!("https://x-access-token:{}@github.com/{}/{}.git", personal_token, GITHUB_USER_NAME, repo);
    execute_git_command(&path, &["remote", "add", "my", &url], true);

    execute_git_command(&path, &["fetch", "my"], true);
    let diff_refspec = format!("HEAD..my/{}", GITHUB_BRANCH_NAME);
    if !git_diff_has_changes(path, &diff_refspec) {
        return false;
    }

    let push_refspec = format!("HEAD:{}", GITHUB_BRANCH_NAME);
    execute_git_command(&path, &["push", "my", &push_refspec, "--force"], true);
    true
}

fn git_diff_has_changes(path: &Path, diff_refspec: &str) -> bool {
    let output = execute_git_command_unchecked(
        path,
        &["diff", "--exit-code", "--name-only", diff_refspec],
    );
    !output.status.success()
}

fn git_status_has_changes(path: &Path) -> bool {
    let output = execute_git_command_unchecked(
        path,
        &["status", "--porcelain"],
    );
    !output.stdout.is_empty()
}

fn execute_git_command_unchecked(path: &Path, args: &[&str]) -> Output {
    Command::new("git")
        .current_dir(path)
        .args(args)
        .output()
        .expect("Failed to execute git command")
}

fn execute_git_command(path: &Path, args: &[&str], panic_if_fail: bool) {
    let output = execute_git_command_unchecked(path, args);
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
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
