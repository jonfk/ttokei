
pub mod text;
pub mod postgres;

use super::git::Commit;

use tokei::Languages;
use chrono::DateTime;
use chrono::FixedOffset;

pub trait Outputter {
    fn start_parse(&self, time: &DateTime<FixedOffset>, git_tag: Option<&str>);
    fn output_tokei<'a>(&self,
                        input: Languages,
                        time: &'a DateTime<FixedOffset>,
                        tag: Option<&'a str>);
    fn output_git<'a>(&self, input: Vec<Commit>, tag: Option<&'a str>);
    fn pre_git_tag_traverse_summary<'a>(&self, origin_remote: &'a str, git_tags: Vec<&'a str>);
    fn should_traverse_tag<'a>(&self, git_tag: &'a str) -> bool;
}
