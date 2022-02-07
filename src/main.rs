use notebook_rs::{argparse, config};

fn main() {
    let matches = argparse::get_args();
    let j = matches.value_of("notebook");
    let notebook = config::read_config(j)
        .expect("Cannot read config")
        .populate_notebook()
        .expect("Error reading entries");

    let args = argparse::parse_args(matches, &notebook);

    let _notebook = notebook
        .run_command(args)
        .expect("Problem running command")
        .write_all_entries()
        .expect("Problem writing all entries");
}
