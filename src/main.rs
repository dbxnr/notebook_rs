use journal::argparse;
use journal::config;
use journal::Args;

fn main() -> std::io::Result<()>{
    let input = argparse::get_args();
    let input = argparse::parse_args(input);

    match input.cmd {
        Args::New(ref n) => input.write_entry(n).expect("Error writing file"),
    };

    config::read_config(None).expect("Cannot read config");

    Ok(())
}
