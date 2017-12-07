
use std::process::Command;

pub fn get_tags() -> Vec<String> {
    let output = Command::new("git")
        .arg("tag")
        .output()
        .expect("git tag failed");
    println!("{:?}", output);

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
    println!("{:?}", output);
    String::from_utf8(output.stdout).expect("get latest git commit date from output")
}

pub fn checkout(rev: &str) {
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
    println!("{:?}", output);
    String::from_utf8(output.stdout).expect("get git current rev from output failed")
}

pub fn checkout_before() {}

pub fn get_rev_before() {}
