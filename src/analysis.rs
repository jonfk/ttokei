
use super::output::Outputter;
use super::git::deprecated as git_dep;
use super::git::get_commits;

use chrono::Duration;
use tokei::Languages;

pub fn analyze_with_tokei<'a, T>(outputer: &T, git_tag: Option<&'a str>)
    where T: Outputter
{
    // Create new Languages
    let mut languages = Languages::new();

    // Get statistics
    languages.get_statistics(vec!["."], vec![]);

    let git_commit_date = git_dep::get_latest_commit_datetime();

    outputer.output_tokei(languages, &git_commit_date, git_tag);
}

pub fn analyze_with_git<'a, T>(outputter: &T, git_tag: Option<&'a str>)
    where T: Outputter
{
    let commits = get_commits(Duration::weeks(2));

    outputter.output_git(commits, git_tag);
}
