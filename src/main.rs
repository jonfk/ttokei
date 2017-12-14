
extern crate ttokei;

extern crate clap;

#[macro_use]
extern crate log;
extern crate env_logger;

use std::env;
use clap::{Arg, ArgGroup, App, SubCommand};

use ttokei::{git, traverse, analysis, output};

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
            .help("Sets the level of verbosity. More v is more verbose"))
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

    match matches.occurrences_of("v") {
        0 => env::set_var("RUST_LOG", "ttokei=error"),
        1 => env::set_var("RUST_LOG", "ttokei=warn"),
        2 => env::set_var("RUST_LOG", "ttokei=info"),
        3 => env::set_var("RUST_LOG", "ttokei=debug"),
        4 | _ => {
            println!("Max logging level");
            env::set_var("RUST_LOG", "ttokei=trace");
        }
    }
    env_logger::init();

    let input = matches.value_of("INPUT").unwrap();
    info!("Using input file: {}", input);


    if matches.is_present("tags") {
        debug!("Traversing git repository over tags");
    } else {
        debug!("ttokei over time");
    }

    if let Some(matches) = matches.subcommand_matches("postgres") {
        let db_url = matches.value_of("url")
            .map(|x| x.to_owned())
            .unwrap_or_else(|| env::var(DB_URL_VAR).expect("No TTOKEI_DB_URL supplied"));

        let outputter = output::postgres::PgOutputter::new(&db_url);
        outputter.run_migrations();

        traverse::run_tags(input, &outputter);

    } else {
        traverse::run_tags(input, &output::text::Text {});
    }
}
