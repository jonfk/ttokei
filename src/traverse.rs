
use super::git::deprecated as git_dep;
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

    let tags = git_dep::get_tags();
    outputter.pre_git_tag_traverse_summary(&git_dep::get_remote_origin_url(),
                                           tags.iter().map(|x| x.as_str()).collect());

    for tag in &tags {
        if outputter.should_traverse_tag(&tag) {
            git_dep::checkout(&tag);
            analysis::analyze_with_tokei(outputter, Some(&tag));
        } else {
            debug!("skipping git tag {} for traversal", tag);
        }
    }

    env::set_current_dir(&prev_path).unwrap();
}
