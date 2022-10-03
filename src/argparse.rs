use crate::{text_from_editor, Args, Entry};
use clap::{Arg, ArgAction, ArgMatches, Command};

pub fn get_args() -> ArgMatches {
    let matches = Command::new("Notebook")
        .about("Note taking")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("new")
                .short_flag('n')
                .long_flag("new")
                .about("Create a new note")
                .arg(Arg::new("entry").multiple_values(true)),
        )
        .arg(
            Arg::new("notebook_name")
                .short('j')
                .long("notebook")
                .takes_value(true)
                .help("Specify a notebook"),
        )
        .subcommand(
            Command::new("list")
                .short_flag('l')
                .long_flag("list")
                .about("List entries")
                .arg(Arg::new("list").takes_value(true).default_value("5")),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .takes_value(false)
                .help("Quantity of information")
                .action(ArgAction::Append),
        )
        .subcommand(
            Command::new("edit")
                .short_flag('e')
                .long_flag("edit")
                .about("Edit specific entry")
                .arg(Arg::new("edit").takes_value(true).max_values(1)),
        )
        .subcommand(
            Command::new("read")
                .short_flag('r')
                .long_flag("read")
                .about("Display specific entry")
                .arg(Arg::new("read").takes_value(true)),
        )
        .subcommand(
            Command::new("delete")
                .short_flag('d')
                .long_flag("delete")
                .about("Delete specific entry")
                .arg(Arg::new("delete").takes_value(true).max_values(1)),
        )
        .subcommand(
            Command::new("search")
                .short_flag('s')
                .long_flag("search")
                .about("Search entries for text")
                .arg(Arg::new("search").takes_value(true)),
        )
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .takes_value(true)
                .help("Path of config file to read"),
        )
        .get_matches();

    matches
}

pub fn parse_args(matches: ArgMatches, dt_format: &str) -> Args {
    let l_verbose = &matches.occurrences_of("verbose");
    let cmd;

    match matches.subcommand() {
        Some(("new", input)) => {
            let s = input
                .get_many::<String>("entry")
                .map(|vals| vals.collect::<Vec<&String>>())
                .unwrap();
            // Below adapted from comment by mdonoughe
            // https://stackoverflow.com/questions/56033289/join-iterator-of-str
            let text = s.into_iter().fold(String::new(), |mut a, b| {
                a.reserve(b.len() + 1);
                a.push_str(b);
                a.push(' ');
                a
            });
            let e = Entry::new(text, dt_format);
            cmd = Args::New(e);
        }

        Some(("list", input)) => {
            let n: usize = input.get_one::<String>("list").unwrap().parse().unwrap();
            cmd = Args::List(n, *l_verbose);
        }

        Some(("read", input)) => {
            let n: usize = input.get_one::<String>("read").unwrap().parse().unwrap();
            cmd = Args::Read(n);
        }

        Some(("edit", input)) => {
            let n: usize = input.get_one::<String>("edit").unwrap().parse().unwrap();
            cmd = Args::Edit(n);
        }

        Some(("delete", input)) => {
            let n: usize = input.get_one::<String>("delete").unwrap().parse().unwrap();
            cmd = Args::Delete(n, true);
        }

        Some(("search", input)) => {
            let q: String = Box::new(input)
                .get_one::<String>("search")
                .unwrap()
                .parse()
                .unwrap();
            cmd = Args::Search(q)
        }

        _ => unreachable!(),
    }

    cmd

    // if matches.is_present("new") {
    //     let text: String = if matches.index_of("new") == None {
    //         text_from_editor(None).unwrap()
    //     } else {
    //         matches
    //             .values_of("new")
    //             .unwrap()
    //             .collect::<Vec<&str>>()
    //             .join(" ")
    //     };
    //     let e = Entry::new(text, dt_format);
    //     cmd = Args::New(e);
    // } else

    // if matches.is_present("list") {
    //     let n = matches
    //         .value_of("list")
    //         .unwrap_or("5")
    //         .parse::<usize>()
    //         .unwrap();
    //     cmd = Args::List(n, *l_verbose);
    // } else

    // if matches.is_present("read") {
    //     let n = matches.value_of("read").unwrap().parse::<usize>().unwrap();
    //     cmd = Args::Read(n);
    // } else

    // if matches.is_present("edit") {
    //     let n = matches.value_of("edit").unwrap().parse::<usize>().unwrap();
    //     cmd = Args::Edit(n);
    // } else

    // if matches.is_present("delete") {
    //     let n = matches
    //         .value_of("delete")
    //         .unwrap()
    //         .parse::<usize>()
    //         .unwrap();
    //     cmd = Args::Delete(n, true);
    // } else

    // if matches.is_present("search") {
    //     let q = Box::new(matches)
    //         .value_of("search")
    //         .unwrap()
    //         .parse()
    //         .unwrap();
    //     cmd = Args::Search(q)
    // };
}
