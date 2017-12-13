
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
    outputter.pre_git_tag_traverse_summary(&git::get_remote_origin_url(),
                                           tags.iter().map(|x| x.as_str()).collect());

    for tag in &tags {
        if outputter.should_traverse_tag(&tag) {
            git::checkout(&tag);
            analysis::get_statistics(outputter, Some(&tag));
        } else {
            debug!("skipping git tag {} for traversal", tag);
        }
    }

    env::set_current_dir(&prev_path).unwrap();
}
