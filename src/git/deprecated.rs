
/// Deprecated functions to be removed eventually and replaced with git2 equivalents

use std::process::Command;
use std::str;
use chrono::DateTime;
use chrono::offset::FixedOffset;

pub fn get_tags() -> Vec<String> {
    let output = Command::new("git")
        .arg("tag")
        .output()
        .expect("git tag failed");
    trace!("git tag output: {:?}", output);

    String::from_utf8(output.stdout)
        .expect("get git tags from output")
        .split("\n")
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

pub fn get_latest_commit_date() -> String {
    //git log -1 --format=%ct
    let output = Command::new("git")
        .args(&["log", "-1", "--format=%ct"])
        .output()
        .expect("get latest git commit date failed");
    trace!("git get latest commit date: {:?}", output);
    String::from_utf8(output.stdout).expect("get latest git commit date from output")
}

pub fn get_latest_commit_datetime() -> DateTime<FixedOffset> {
    //git log -1 --format=%cI
    let mut output = Command::new("git")
        .args(&["log", "-1", "--format=%cI"])
        .output()
        .expect("get latest git commit datetime failed");
    trace!("git get latest commit datetime: {:?}", output);
    output.stdout.pop();
    DateTime::parse_from_str(str::from_utf8(&output.stdout)
                                 .expect("get latest git commit datetime from output"),
                             "%+")
        .expect("get latest git commit datetime chrono datetime")
}

pub fn checkout(rev: &str) {
    debug!("git checkout {}", rev);
    Command::new("git")
        .arg("checkout")
        .arg(rev)
        .output()
        .expect("git checkout failed");
}

pub fn get_git_current_rev() -> String {
    let output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .expect("get git current rev failed");
    trace!("git get current rev: {:?}", output);
    String::from_utf8(output.stdout).expect("get git current rev from output failed")
}

pub fn get_remote_origin_url() -> String {
    let output = Command::new("git")
        .args(&["config", "--get", "remote.origin.url"])
        .output()
        .expect("get git remote_origin_url failed");
    trace!("git get git remote_origin_url: {:?}", output);
    String::from_utf8(output.stdout).expect("git get git remote_origin_url from output failed")
}

pub fn checkout_before() {}

pub fn get_rev_before() {}
