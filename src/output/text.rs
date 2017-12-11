const ROW: &'static str = "----------------------------------------------------\
                           ---------------------------";

use super::Outputter;

use std::borrow::Cow;
use tokei::{Languages, LanguageType, Language};
use chrono::{DateTime, FixedOffset};

#[derive(Copy, Clone)]
pub struct Text {}

impl Outputter for Text {
    fn output<'a>(&self,
                  languages: Languages,
                  time: &'a DateTime<FixedOffset>,
                  tag: Option<&'a str>) {
        println!("{}", ROW);
        println!("Time: {}, Git Tag: {:?}", time, tag);
        println!(" {:<12} {:>12} {:>12} {:>12} {:>12} {:>12}",
                 "Language",
                 "Files",
                 "Lines",
                 "Code",
                 "Comments",
                 "Blanks");
        println!("{}", ROW);

        for (name, language) in languages.iter().filter(isnt_empty) {
            print_language(language, name);
        }

        println!("{}", ROW);

        let mut total = Language::new_blank();
        for (_, language) in languages {
            total += language;
        }
        println!(" {: <18} {: >6} {:>12} {:>12} {:>12} {:>12}",
                 "Total",
                 total.stats.len(),
                 total.lines,
                 total.code,
                 total.comments,
                 total.blanks);
        println!("{}", ROW);
    }

    fn pre_git_tag_traverse_summary<'a>(&self, origin_remote: &'a str, git_tags: Vec<&'a str>) {
        println!("{}", ROW);
        println!("Repository: {}", origin_remote);
    }
}


fn print_language<'a, C>(language: &'a Language, name: C)
    where C: Into<Cow<'a, LanguageType>>
{
    println!(" {: <18} {: >6} {:>12} {:>12} {:>12} {:>12}",
             name.into().name(),
             language.stats.len(),
             language.lines,
             language.code,
             language.comments,
             language.blanks)
}

fn isnt_empty(&(_, language): &(&LanguageType, &Language)) -> bool {
    !language.is_empty()
}
