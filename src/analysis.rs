
use super::output::Outputter;
use super::git;

use tokei::Languages;

pub fn get_statistics<'a, T>(outputer: &T, git_tag: Option<&'a str>)
    where T: Outputter
{
    // Create new Languages
    let mut languages = Languages::new();

    // Get statistics
    languages.get_statistics(vec!["."], vec![]);

    let git_commit_date = git::get_latest_commit_datetime();

    outputer.output(languages, &git_commit_date, git_tag);
}
