
use super::schema::parses::dsl::*;

use diesel::pg::PgConnection;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::select;

pub fn does_parse_exist(conn: &PgConnection, git_tag_query: &str) -> bool {
    select(exists(parses.filter(git_tag.eq(git_tag_query))))
        .get_result(conn)
        .expect("does parse exist")
}
