use crate::{config, text_from_editor, Args, Entry};
use clap::{App, AppSettings, Arg, ArgMatches};

pub fn get_args() -> ArgMatches<'static> {
    let matches = App::new("Journal")
        .version("0.1.0")
        .author("")
        .about("Note taking")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::with_name("new")
                .short("n")
                .long("new")
                .takes_value(true)
                .min_values(0)
                .required(false)
                .help("Create a new note")
                .multiple(true),
        )
        .arg(
            Arg::with_name("journal")
                .short("j")
                .long("journal")
                .takes_value(true)
                .required(false)
                .help("Specify a journal")
                .multiple(false),
        )
        .get_matches();

    matches
}

pub fn parse_args(matches: ArgMatches) -> Args {
    let j = matches.value_of("journal");

    let journal = config::read_config(j).expect("Cannot read config");

    let mut cmd = Args::New;
    let mut text = String::new();

    if matches.is_present("new") {
        cmd = Args::New;

        text = if matches.index_of("new") == None {
            text_from_editor().unwrap()
        } else {
            matches
                .values_of("new")
                .unwrap()
                .collect::<Vec<&str>>()
                .join(" ")
        }
    }

    let e = Entry::new(text, &journal.dt_format);

    cmd(journal, e)
}
