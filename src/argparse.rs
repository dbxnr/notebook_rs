use crate::{text_from_editor, Args, Journal, NewEntry};
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

pub fn parse_args(matches: ArgMatches) -> Journal {
    let mut cmd = Args::New;
    let mut text = String::new();
    let mut filename = String::new();
    let dt = Local::now();

    if matches.is_present("new") {
        cmd = Args::New;

        text = if matches.index_of("new") == None {
            text_from_editor()
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

    let mut e = NewEntry::new(Some(text), dt);
    e.calculate_sentiment();

    let i = Journal::new(&cmd(e), Some(filename));

    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_no_editor() {}
}
