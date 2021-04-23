use crate::{config, text_from_editor, Args, Entry};
use clap::{App, AppSettings, Arg, ArgMatches};
use std::io;

pub fn get_args() -> ArgMatches<'static> {
    let matches = App::new("Notebook")
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
            Arg::with_name("notebook")
                .short("j")
                .long("notebook")
                .takes_value(true)
                .help("Specify a notebook"),
        )
        .arg(
            Arg::with_name("list")
                .short("l")
                .long("list")
                .takes_value(true)
                .min_values(0)
                .help("List entries"),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .takes_value(false)
                .help("Quantity of information")
                .multiple(true),
        )       
        .arg(
            Arg::with_name("read")
                .short("r")
                .long("read")
                .takes_value(true)
                .help("Display specific entry"),
        )
        .get_matches();

    matches
}

pub fn parse_args(matches: ArgMatches) {
    let j = matches.value_of("notebook");

    let mut notebook = config::read_config(j).expect("Cannot read config");

    if matches.is_present("new") {
        let text = if matches.index_of("new") == None {
            text_from_editor().unwrap()
        } else {
            matches
                .values_of("new")
                .unwrap()
                .collect::<Vec<&str>>()
                .join(" ")
        };
        let e = Entry::new(text, &notebook.dt_format);
        let cmd = Args::New(&notebook, e);
        run_command(cmd)
    };

    if matches.is_present("list") {
        //TODO: Add default value
        let n = matches
            .value_of("list")
            .unwrap_or("5")
            .parse::<usize>()
            .unwrap();
        notebook.read_entries().expect("Error reading entries");
        let cmd = Args::List(&notebook, n);
        run_command(cmd)
    };

    if matches.is_present("read") {
        let n = matches.value_of("read").unwrap().parse::<usize>().unwrap();
        notebook.read_entries().expect("Error reading entries");
        let cmd = Args::Read(&notebook, n);
        run_command(cmd)
    };
}

fn run_command(cmd: Args) {
    match cmd {
        Args::New(j, ref e) => j.write_entry(e),
        Args::List(j, ref n) => j.list_entries(n, &mut io::stdout()),
        Args::Read(j, ref n) => j.read_entry(n, &mut io::stdout()),
    }
    .expect("Error matching command");
}
