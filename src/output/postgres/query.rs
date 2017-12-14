
use super::schema::parses::dsl::*;
use super::model::Parse;

use diesel::pg::PgConnection;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::select;

pub fn does_parse_exist(conn: &PgConnection, git_tag_query: &str) -> bool {
    select(exists(parses.filter(git_tag.eq(git_tag_query))))
        .get_result(conn)
        .expect("does parse exist")
}

pub fn find_parse(conn: &PgConnection, git_tag_query: &str) -> Option<i32> {
    use super::schema::parses::dsl::*;

    parses.filter(git_tag.eq(git_tag_query))
        .select(parse_id)
        .get_result(conn)
        .optional()
        .expect("find parse")
}

pub fn is_parse_completed(conn: &PgConnection, git_tag_query: &str) -> bool {
    use super::schema::completed_parses;

    let result = select(exists(parses.inner_join(completed_parses::table)
            .filter(git_tag.eq(git_tag_query))))
        .get_result(conn)
        .expect("is parse completed");
    debug!("is_parse_completed = {}", result);
    result
}
