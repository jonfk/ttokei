
pub mod model;
pub mod schema;
pub mod insert;

use super::Outputter;

use self::model::{NewParse, NewLanguage, NewLanguageStats};

use std;
use log::Level;
use tokei::Languages;
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
}

impl Outputter for PgOutputter {
    fn output<'a>(&self,
                  languages: Languages,
                  time: &'a DateTime<FixedOffset>,
                  git_tag: Option<&'a str>) {
        let parse_id = insert::create_parse(&self.conn,
                                            NewParse {
                                                time: time,
                                                git_tag: git_tag,
                                            });
        debug!("inserted parse: {}", parse_id);

        let language_map = languages.remove_empty();

        for (name, language) in language_map {
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

            for stats in language.stats {
                let language_stats_id =
                    insert::create_language_stats(&self.conn,
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
                                                  });
                debug!("inserted language stats {}", language_stats_id);
            }
        }
    }
}
