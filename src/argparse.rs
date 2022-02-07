use crate::{text_from_editor, Args, Entry, Notebook};
use clap::{App, AppSettings, Arg, ArgMatches};

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
        .arg(
            Arg::new("delete")
                .short('d')
                .long("delete")
                .takes_value(true)
                .help("Delete specific entry"),
        )
        .get_matches();

    matches
}

pub fn parse_args(matches: ArgMatches, notebook: &Notebook) -> Args {
    let l_verbose = &matches.occurrences_of("verbose");
    let mut cmd = Args::Unimplemented();

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
        cmd = Args::New(e);
    } else if matches.is_present("list") {
        let n = matches
            .value_of("list")
            .unwrap_or("5")
            .parse::<usize>()
            .unwrap();
        cmd = Args::List(n, *l_verbose);
    } else if matches.is_present("read") {
        let n = matches.value_of("read").unwrap().parse::<usize>().unwrap();
        cmd = Args::Read(n);
    } else if matches.is_present("edit") {
        let n = matches.value_of("edit").unwrap().parse::<usize>().unwrap();
        cmd = Args::Edit(n);
    } else if matches.is_present("delete") {
        let n = matches
            .value_of("delete")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        cmd = Args::Delete(n, true);
    };

    cmd
}
