use std::{
    env::{temp_dir, var},
    error::Error,
    fs,
    io::prelude::*,
    process::Command,
};

pub mod argparse;


pub fn write_entry(input: UserInput) -> Result<(), Box<dyn Error>> {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(input.filename)?;

    file.write_all(input.text.as_bytes())?;
    file.write_all(b"\n\n")?;

    write_to_temp();

    Ok(())
}

fn write_to_temp() -> std::io::Result<()> {
    let editor = var("EDITOR").unwrap();
    let mut file_path = temp_dir();
    file_path.push("editable");
    fs::File::create(&file_path).expect("Could not create file.");

    Command::new(editor)
        .arg(&file_path)
        .status()
        .expect("Something went wrong with the editor.");

    let mut editable = String::new();
    fs::File::open(&file_path)
        .expect("Couldn't open temp file.")
        .read_to_string(&mut editable);

    println!("File contents: {}", editable);
    fs::remove_file(file_path)?;

    Ok(())
}

pub struct UserInput {
    filename: String,
    text: String,
}

impl UserInput {
    pub fn new(args: &[String]) -> Result<UserInput, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }

        let filename = args[1].clone();
        let text = args[2].clone();

        Ok(UserInput { filename, text })
    }
}

