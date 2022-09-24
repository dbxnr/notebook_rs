use notebook_rs::{argparse, config};

fn main() {
    let matches = argparse::get_args();
    let j = matches.value_of("notebook_name");
    let c = matches.value_of("config");
    let notebook = config::read_config(j, c).expect("Cannot read config");
    config::check_create_file(&notebook.file).expect("Error reading notebook file.");

    let args = argparse::parse_args(matches, &notebook.dt_format);

    notebook
        .populate_notebook()
        .expect("Error populating notebook")
        .run_command(args)
        .expect("Problem running command")
        .write_all_entries()
        .expect("Problem writing all entries");
}
