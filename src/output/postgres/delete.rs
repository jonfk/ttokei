
use diesel;
use diesel::pg::PgConnection;
use diesel::RunQueryDsl;
use diesel::dsl::*;
use diesel::prelude::*;

use super::model::*;

pub fn parse_by_git_tag(conn: &PgConnection, git_tag: &str) {
    use super::schema::parses;

    diesel::delete(parses::table.filter(parses::git_tag.eq(git_tag)))
        .execute(conn)
        .expect("delete parse by git tag");
}
