
use super::output::Outputter;
use super::git::deprecated as git_dep;
use super::git::get_commits;

use chrono::{Duration, DateTime, FixedOffset};
use tokei::Languages;

pub fn analyze<'a, T>(outputter: &T,
                      git_commit_date: &DateTime<FixedOffset>,
                      git_tag: Option<&'a str>)
    where T: Outputter
{
    outputter.start_parse(&git_commit_date, git_tag);
    analyze_with_tokei(outputter, &git_commit_date, git_tag);
    analyze_with_git(outputter, git_tag);
}

pub fn analyze_with_tokei<'a, T>(outputer: &T,
                                 git_commit_date: &DateTime<FixedOffset>,
                                 git_tag: Option<&'a str>)
    where T: Outputter
{
    // Create new Languages
    let mut languages = Languages::new();

    // Get statistics
    languages.get_statistics(vec!["."], vec![]);

    outputer.output_tokei(languages, git_commit_date, git_tag);
}

pub fn analyze_with_git<'a, T>(outputter: &T, git_tag: Option<&'a str>)
    where T: Outputter
{
    let commits = get_commits(Duration::weeks(2));

    outputter.output_git(commits, git_tag);
}
