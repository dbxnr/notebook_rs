use crate::{Args, UserInput};
use chrono::prelude::Local;
use clap::{App, Arg, ArgMatches};

pub fn get_args() -> ArgMatches<'static> {
    let matches = App::new("Journal")
        .version("0.1.0")
        .author("")
        .about("Note taking")
        .arg(
            Arg::with_name("new")
                .short("n")
                .long("add")
                .takes_value(true)
                .min_values(0)
                .required(false)
                .help("Create a new note")
                .multiple(true),
        )
        .get_matches();

    matches
}

pub fn parse_args(matches: ArgMatches) -> UserInput {
    let mut cmd = Args::New;
    let mut text = String::new();
    let mut filename = String::new();
    let dt = Local::now();

    if matches.is_present("new") {
        cmd = Args::New;

        text = if matches.index_of("new") == None {
            "none".to_string()
        } else {
            matches
                .values_of("new")
                .unwrap()
                .collect::<Vec<&str>>()
                .join(" ")
        }
        .to_string();

        filename = "_test.txt".to_string();
    }

    let i = UserInput::new(cmd, Some(text), Some(filename), dt);

    i
}
