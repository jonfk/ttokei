
use super::schema::parses;
use super::schema::languages;
use super::schema::language_stats;
use chrono::DateTime;
use chrono::offset::FixedOffset;
//use diesel::types::ToSql;
//use diesel::pg::types::sql_types::Timestamptz;

#[derive(Insertable)]
#[table_name="parses"]
pub struct NewParse<'a> {
    pub time: &'a DateTime<FixedOffset>,
    pub git_tag: Option<&'a str>,
}

#[derive(Insertable)]
#[table_name="languages"]
pub struct NewLanguage<'a> {
    pub parse_id: i32,
    pub name: &'a str,
    pub blanks: i64,
    pub code: i64,
    pub comments: i64,
    pub lines: i64,
    pub nested: bool, 
    // pub files: Vec<&'a str>,
    // pub line_comment: Vec<&'a str>,
    // pub multi_line: Vec<Vec<&'a str>>,
    // pub nested_comments: Vec<Vec<&'a str>>,
    // pub quotes: Vec<Vec<&'a str>>,
}

#[derive(Insertable)]
#[table_name="language_stats"]
pub struct NewLanguageStats<'a> {
    pub language_id: i64,
    pub name: &'a str,
    pub blanks: i64,
    pub code: i64,
    pub comments: i64,
    pub lines: i64,
}
