use crate::{text_from_editor, Args, Entry};
use clap::{Arg, ArgMatches, Command};

pub fn get_args() -> ArgMatches {
    Command::new("Notebook")
        .about("CLI utility for plaintext notetaking.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("new")
                .short_flag('n')
                .long_flag("new")
                .about("Create a new note")
                .arg(Arg::new("entry")),
        )
        .arg(
            Arg::new("notebook_name")
                .short('j')
                .long("notebook")
                .default_value("default")
                .help("Specify a notebook name"),
        )
        .subcommand(
            Command::new("list")
                .short_flag('l')
                .long_flag("list")
                .about("List entries")
                .arg(Arg::new("list").default_value("5")),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Quantity of information")
                .action(clap::ArgAction::Count),
        )
        .subcommand(
            Command::new("edit")
                .short_flag('e')
                .long_flag("edit")
                .about("Edit specific entry")
                .arg(Arg::new("edit")),
        )
        .subcommand(
            Command::new("read")
                .short_flag('r')
                .long_flag("read")
                .about("Display specific entry")
                .arg(Arg::new("read")),
        )
        .subcommand(
            Command::new("delete")
                .short_flag('d')
                .long_flag("delete")
                .about("Delete specific entry")
                .arg(Arg::new("delete")),
        )
        .subcommand(
            Command::new("search")
                .short_flag('s')
                .long_flag("search")
                .about("Query to search, enclosed in quotations")
                .arg(Arg::new("search"))
                .subcommand(
                    Command::new("date")
                        .short_flag('d')
                        .long_flag("date")
                        .about("Search by date range")
                        .arg(Arg::new("entry")),
                ),
        )
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .help("Path of config file to read"),
        )
        .get_matches()
}

pub fn parse_args(matches: ArgMatches, dt_format: &str) -> Args {
    let verbose = matches.get_count("verbose");

    match matches.subcommand() {
        Some(("new", input)) => {
            let text: String = match input.get_many::<String>("entry") {
                Some(t) => t.fold(String::new(), |mut a, b| {
                    a.reserve(b.len() + 1);
                    a.push_str(b);
                    a.push(' ');
                    a
                }),

                None => text_from_editor(None).unwrap(),
            };
            let e = Entry::new(text, dt_format);
            Args::New(e)
        }

        Some(("list", input)) => {
            let n: usize = input.get_one::<String>("list").unwrap().parse().unwrap();
            Args::List(n, verbose)
        }

        Some(("read", input)) => {
            let n: usize = input.get_one::<String>("read").unwrap().parse().unwrap();
            Args::Read(n)
        }

        Some(("edit", input)) => {
            let n: usize = input.get_one::<String>("edit").unwrap().parse().unwrap();
            Args::Edit(n)
        }

        Some(("delete", input)) => {
            let n: usize = input.get_one::<String>("delete").unwrap().parse().unwrap();
            Args::Delete(n, true)
        }

        Some(("search", input)) => {
            let q: String = Box::new(input)
                .get_one::<String>("search")
                .unwrap()
                .parse()
                .unwrap();
            Args::Search(q)
        }

        _ => unreachable!(),
    }
}
