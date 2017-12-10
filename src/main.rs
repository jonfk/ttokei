
extern crate tokei;
extern crate clap;
extern crate chrono;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_infer_schema;

pub mod git;
pub mod traverse;
pub mod analysis;
pub mod output;

use clap::{Arg, ArgGroup, App, SubCommand};

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
            .arg(Arg::with_name("username")
                .short("u")
                .long("username")
                .help("Postgres username"))
            .arg(Arg::with_name("password")
                .short("p")
                .long("password")
                .help("Postgres password")))
        .get_matches();

    let input = matches.value_of("INPUT").unwrap();
    println!("Using input file: {}", input);

    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }
    if matches.is_present("tags") {
        println!("tags");
    } else {
        println!("ttokei over time");
    }

    if let Some(matches) = matches.subcommand_matches("postgres") {
    } else {
        traverse::run_tags(input, &output::text::Text {});
        //analysis::get_statistics(output::text::Text {}, "".to_string())
    }
}
