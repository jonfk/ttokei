
use super::git;
use super::analysis;
use super::output::Outputter;

use std::env;
use std::path::Path;

pub fn run_tags<T>(input_path: &str, outputter: &T)
    where T: Outputter
{
    let prev_path = env::current_dir().unwrap();
    let new_path = Path::new(&input_path);
    env::set_current_dir(&new_path).unwrap();

    let tags = git::get_tags();

    for tag in &tags {
        git::checkout(&tag);
        let last_date = git::get_latest_commit_date();
        analysis::get_statistics(outputter, last_date);
    }

    env::set_current_dir(&prev_path).unwrap();
}