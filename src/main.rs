use notebook_rs::{argparse, config};

fn main() {
    let matches = argparse::get_args();
    let j: &String = matches
        .get_one("notebook_name")
        .expect("Error getting notebook name.");
    let c = matches.try_get_one("config").unwrap();
    let notebook = config::read_config(j, c).expect("Error reading config file.");
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
