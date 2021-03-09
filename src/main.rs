use journal::argparse;
use journal::Args;

fn main() {
    let input = argparse::get_args();
    let input = argparse::parse_args(input);

    match input.cmd {
        Args::New(ref n) => input.write_entry(n).expect("Error writing file"),
    };
}
