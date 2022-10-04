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

    let cmd = match matches.subcommand() {
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
            Args::List(n, *l_verbose)
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
    };

    cmd
}
