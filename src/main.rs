use std::env;
use std::process;

use journal::argparse;
use journal::UserInput;

fn main() {
    let args: Vec<String> = argparse::parse_args();

    let input = UserInput::new(&args)
        .unwrap_or_else(|err| {
            println!("Problem parsing arguments: {}", err);
            process::exit(1);
        });

    if let Err(e) = journal::write_entry(input) {
        println!("Application error: {}", e);

        process::exit(1);
    };
}

