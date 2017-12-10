
extern crate tokei;
extern crate clap;
extern crate chrono;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;


pub mod git;
pub mod traverse;
pub mod analysis;
pub mod output;

use std::env;
use clap::{Arg, ArgGroup, App, SubCommand};

static DB_URL_VAR: &'static str = "TTOKEI_DB_URL";

fn main() {
    let matches = App::new("Tokei over time")
        .version("1.0")
        .author("Jonathan Fok kan<jfokkan@gmail.com>")
        .about("Produces code statistics from tokei over a git repository")
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to use")
            .required(true)
            .index(1))
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .arg(Arg::with_name("tags")
            .short("a")
            .long("tags")
            .takes_value(false)
            .help("Runs the program on tags"))
        .arg(Arg::with_name("time")
            .short("t")
            .long("time")
            .takes_value(true)
            .help("Runs the program over time"))
        .group(ArgGroup::with_name("traverse")
            .args(&["tags", "time"])
            .required(true))
        .subcommand(SubCommand::with_name("postgres")
            .about("Outputs the results to postgres")
            .arg(Arg::with_name("url")
                .short("u")
                .long("url")
                .takes_value(true)
                .help("Postgres database url can also be set with TTOKEI_DB_URL env var")))
        .get_matches();

    let input = matches.value_of("INPUT").unwrap();
    println!("Using input file: {}", input);

    let is_verbose = matches.is_present("v");

    if matches.is_present("tags") {
        println!("tags");
    } else {
        println!("ttokei over time");
    }

    if let Some(matches) = matches.subcommand_matches("postgres") {
        let db_url = matches.value_of("url")
            .map(|x| x.to_owned())
            .unwrap_or_else(|| env::var(DB_URL_VAR).expect("No TTOKEI_DB_URL supplied"));

        let outputter = output::postgres::PgOutputter::new(&db_url);
        outputter.run_migrations(is_verbose);

        traverse::run_tags(input, &outputter);

    } else {
        traverse::run_tags(input, &output::text::Text {});
    }
}
