
pub mod text;
pub mod postgres;

use tokei::{Languages, LanguageType};
use chrono::DateTime;
use chrono::FixedOffset;

pub trait Outputter {
    fn output<'a>(&self, input: Languages, time: &'a DateTime<FixedOffset>, tag: Option<&'a str>);
}
