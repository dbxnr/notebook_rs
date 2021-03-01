use clap::{Arg, App, ArgMatches};

fn parse_args() {
    let matches = App::new("Journal")
        .version("0.1.0")
        .author("")
        .about("Note taking")
        .arg(Arg::with_name("new")
             .short("+")
             .long("add")
             .takes_value(true)
             .help("Create a new note"))
        .get_matches();
}
