
pub mod model;
pub mod schema;
pub mod insert;
pub mod query;
pub mod delete;

use super::Outputter;

use self::model::{NewParse, NewLanguage, NewLanguageStats, NewGitRepo, NewGitTag};

use std;
use log::Level;
use tokei::{Languages, Stats, LanguageType, Language};
use diesel::pg::PgConnection;
use diesel::Connection;
use chrono::{DateTime, FixedOffset};

embed_migrations!("./migrations");

pub struct PgOutputter {
    conn: PgConnection,
}

impl PgOutputter {
    pub fn new(db_url: &str) -> PgOutputter {
        PgOutputter {
            conn: PgConnection::establish(&db_url)
                .expect(&format!("Error connecting to {}", db_url)),
        }
    }

    pub fn run_migrations(&self) {
        if log_enabled!(Level::Info) {
            embedded_migrations::run_with_output(&self.conn, &mut std::io::stdout())
                .expect("postgres run migrations");
        } else {
            embedded_migrations::run(&self.conn).expect("postgres run migrations");
        }
    }

    pub fn insert_language(&self, parse_id: i32, name: LanguageType, language: Language) {
        let language_id = insert::create_language(&self.conn,
                                                  NewLanguage {
                                                      parse_id: parse_id,
                                                      name: name.name(),
                                                      blanks: language.blanks as i64,
                                                      code: language.code as i64,
                                                      comments: language.comments as i64,
                                                      lines: language.lines as i64,
                                                      nested: language.nested,
                                                  });
        debug!("inserted language: {}", language_id);

        self.insert_language_stats(parse_id, language_id, language.stats);
    }

    pub fn insert_language_stats(&self,
                                 parse_id: i32,
                                 language_id: i64,
                                 language_stats: Vec<Stats>) {
        let bulk_stats_values: Vec<_> = language_stats.iter()
            .map(|stats| {
                NewLanguageStats {
                    language_id: language_id,
                    parse_id: parse_id,
                    name: stats.name
                        .to_str()
                        .expect("stats name is not utf8"),
                    blanks: stats.blanks as i64,
                    code: stats.code as i64,
                    comments: stats.comments as i64,
                    lines: stats.lines as i64,
                }
            })
            .collect();


        if bulk_stats_values.len() > 65534 {
            for stats_values in bulk_stats_values.chunks(65534) {
                let language_stats_id = insert::create_language_stats(&self.conn,
                                                                      stats_values.to_vec());
                debug!("inserted language stats {}", language_stats_id);
            }
        } else {
            let language_stats_id = insert::create_language_stats(&self.conn, bulk_stats_values);
            debug!("inserted language stats {}", language_stats_id);
        }
    }
}

impl Outputter for PgOutputter {
    fn output<'a>(&self,
                  languages: Languages,
                  time: &'a DateTime<FixedOffset>,
                  git_tag: Option<&'a str>) {
        let git_tag = git_tag.expect("git_tag should exist TODO remove assumption");
        if query::does_parse_exist(&self.conn, git_tag) &&
           !query::is_parse_completed(&self.conn, git_tag) {
            delete::parse_by_git_tag(&self.conn, git_tag);

            debug!("deleted incomplete parse with tag {}", git_tag);
        }
        let parse_id = insert::create_parse(&self.conn,
                                            NewParse {
                                                time: time,
                                                git_tag: Some(git_tag),
                                            });
        debug!("inserted parse: {}", parse_id);

        let language_map = languages.remove_empty();

        for (name, language) in language_map {
            self.insert_language(parse_id, name, language);
        }
        insert::create_completed_parse(&self.conn, parse_id);
    }


    fn pre_git_tag_traverse_summary<'a>(&self, origin_remote: &'a str, git_tags: Vec<&'a str>) {
        if let Some(git_repo_id) =
            insert::create_git_repo(&self.conn, NewGitRepo { origin_remote: origin_remote }).ok() {

            debug!("inserted git repo {}", git_repo_id);

            let bulk_tag_values = git_tags.iter()
                .map(|tag| {
                    NewGitTag {
                        git_repo_id: git_repo_id,
                        git_tag: tag,
                    }
                })
                .collect();
            if let Some(git_tag_id) = insert::create_git_tags(&self.conn, bulk_tag_values).ok() {
                debug!("inserted git tag {}", git_tag_id);
            }
        }
    }

    fn should_traverse_tag<'a>(&self, git_tag: &'a str) -> bool {
        !query::is_parse_completed(&self.conn, git_tag)
    }
}
