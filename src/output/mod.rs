
pub mod text;
pub mod postgres;

use tokei::Languages;
use chrono::DateTime;
use chrono::FixedOffset;

pub trait Outputter {
    fn output<'a>(&self, input: Languages, time: &'a DateTime<FixedOffset>, tag: Option<&'a str>);
    fn pre_git_tag_traverse_summary<'a>(&self, origin_remote: &'a str, git_tags: Vec<&'a str>);
}
