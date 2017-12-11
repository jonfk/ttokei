
use diesel;
use diesel::pg::PgConnection;
use diesel::RunQueryDsl;

use super::model::*;

pub fn create_parse<'a>(conn: &PgConnection, new_parse: NewParse<'a>) -> i32 {
    use super::schema::parses;

    diesel::insert_into(parses::table)
        .values(&new_parse)
        .returning(parses::parse_id)
        .get_result(conn)
        .expect("create_parse execute")
}

pub fn create_language<'a>(conn: &PgConnection, new_language: NewLanguage<'a>) -> i64 {
    use super::schema::languages;

    diesel::insert_into(languages::table)
        .values(&new_language)
        .returning(languages::language_id)
        .get_result(conn)
        .expect("create_language execute")
}

pub fn create_language_stats<'a>(conn: &PgConnection,
                                 new_language_stat: Vec<NewLanguageStats<'a>>)
                                 -> i64 {
    use super::schema::language_stats;

    diesel::insert_into(language_stats::table)
        .values(&new_language_stat)
        .returning(language_stats::language_stat_id)
        .get_result(conn)
        .expect("create_language_stat execute")
}

pub fn create_git_repo<'a>(conn: &PgConnection, new_git_repo: NewGitRepo<'a>) -> i32 {
    use super::schema::git_repos;

    diesel::insert_into(git_repos::table)
        .values(&new_git_repo)
        .returning(git_repos::git_repo_id)
        .get_result(conn)
        .expect("create_git_repo execute")
}

pub fn create_git_tags<'a>(conn: &PgConnection, new_git_tags: Vec<NewGitTag<'a>>) -> i64 {
    use super::schema::git_tags;

    diesel::insert_into(git_tags::table)
        .values(&new_git_tags)
        .returning(git_tags::git_tag_id)
        .get_result(conn)
        .expect("create_git_tag execute")
}
