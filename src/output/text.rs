const BLANKS: &'static str = "blanks";
const CODE: &'static str = "code";
const COMMENTS: &'static str = "comments";
const FILES: &'static str = "files";
const LINES: &'static str = "lines";
const ROW: &'static str = "----------------------------------------------------\
                           ---------------------------";

use tokei::{Languages, LanguageType, Language};
use std::borrow::Cow;

use super::Outputter;

#[derive(Copy, Clone)]
pub struct Text {}

impl Outputter for Text {
    fn output(&self, languages: Languages) {
        println!("{}", ROW);
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
