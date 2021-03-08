use journal::argparse;

fn main() {
    let input = argparse::get_args();
    let input = argparse::parse_args(input);

    input.write_entry().expect("Error writing entry");
}
