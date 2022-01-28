use crate::{config, text_from_editor, Args, Entry};
use clap::{App, AppSettings, Arg, ArgMatches};
use std::io;

pub fn get_args() -> ArgMatches {
    let matches = App::new("Notebook")
        .version("0.2.1")
        .author("")
        .about("Note taking")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::new("new")
                .short('n')
                .long("new")
                .takes_value(true)
                .min_values(0)
                .required(false)
                .help("Create a new note")
                .multiple_values(true)
                .conflicts_with_all(&["list", "verbose", "edit", "read"]),
        )
        .arg(
            Arg::new("notebook")
                .short('j')
                .long("notebook")
                .takes_value(true)
                .help("Specify a notebook"),
        )
        .arg(
            Arg::new("list")
                .short('l')
                .long("list")
                .takes_value(true)
                .min_values(0)
                .help("List entries")
                .conflicts_with_all(&["edit", "read"]),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .takes_value(false)
                .help("Quantity of information")
                .multiple_occurrences(true),
        )
        .arg(
            Arg::new("edit")
                .short('e')
                .long("edit")
                .takes_value(true)
                .max_values(1)
                .help("Edit specific entry")
                .conflicts_with_all(&["read"]),
        )
        .arg(
            Arg::new("read")
                .short('r')
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
    let l_verbose = &matches.occurrences_of("verbose");

    if matches.is_present("new") {
        let text: String = if matches.index_of("new") == None {
            text_from_editor(None).unwrap()
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
        let cmd = Args::List(&notebook, n, *l_verbose);
        run_command(cmd)
    };

    if matches.is_present("read") {
        let n = matches.value_of("read").unwrap().parse::<usize>().unwrap();
        notebook.read_entries().expect("Error reading entries");
        let cmd = Args::Read(&notebook, n);
        run_command(cmd)
    };

    if matches.is_present("edit") {
        let n = matches.value_of("edit").unwrap().parse::<usize>().unwrap();
        notebook.read_entries().expect("Error reading entries");
        let cmd = Args::Edit(notebook, n);
        run_command(cmd)
    };
}

fn run_command(cmd: Args) {
    match cmd {
        Args::New(j, ref e) => j.write_entry(e, None),
        Args::List(j, ref n, l) => j.list_entries(n, &mut io::stdout(), l),
        Args::Read(j, ref n) => j.read_entry(n, &mut io::stdout()),
        Args::Edit(mut j, n) => j.edit_entry(n),
    }
    .expect("Error matching command");
}
