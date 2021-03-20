use journal::argparse;
use journal::Args;

fn main() -> std::io::Result<()> {
    let input = argparse::get_args();
    let input = argparse::parse_args(input);

    match input {
        Args::New(ref j, ref e) => j.write_entry(e).expect("Error writing file"),
    };

    Ok(())
}
