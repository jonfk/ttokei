
use super::schema::*;
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

#[derive(Insertable, Clone)]
#[table_name="language_stats"]
pub struct NewLanguageStats<'a> {
    pub language_id: i64,
    pub parse_id: i32,
    pub name: &'a str,
    pub blanks: i64,
    pub code: i64,
    pub comments: i64,
    pub lines: i64,
}

#[derive(Insertable)]
#[table_name="git_repos"]
pub struct NewGitRepo<'a> {
    pub origin_remote: &'a str,
}

#[derive(Insertable)]
#[table_name="git_tags"]
pub struct NewGitTag<'a> {
    pub git_repo_id: i32,
    pub git_tag: &'a str,
}

#[derive(Insertable)]
#[table_name="completed_parses"]
pub struct NewCompletedParse {
    pub parse_id: i32,
}


#[derive(Queryable)]
pub struct Parse {
    pub parse_id: i32,
    pub time: DateTime<FixedOffset>,
    pub git_tag: Option<String>,
}

#[derive(Queryable)]
pub struct CompletedParse {
    pub completed_parse_id: i32,
    pub parse_id: i32,
}
