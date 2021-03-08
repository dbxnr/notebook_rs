use chrono::prelude::{DateTime, Local};
use std::{
    env::{temp_dir, var},
    error::Error,
    fs,
    io::prelude::*,
    process::Command,
};

pub mod argparse;

#[derive(Debug)]
pub enum Args {
    New,
}

#[derive(Debug)]
pub struct UserInput {
    cmd: Args,
    text: Option<String>,
    filename: Option<String>,
    timestamp: DateTime<Local>,
}

impl UserInput {
    fn new(
        cmd: Args,
        text: Option<String>,
        filename: Option<String>,
        timestamp: DateTime<Local>,
    ) -> UserInput {
        UserInput {
            cmd: cmd,
            text: text,
            filename: filename,
            timestamp: timestamp,
        }
    }

    pub fn write_entry(&self) -> Result<(), Box<dyn Error>> {
        let mut file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open("_test.txt")?;

        file.write_all(self.timestamp.to_string().as_bytes())?;
        file.write_all(b"\n")?;
        file.write_all(self.text.as_ref().unwrap().as_bytes())?;
        file.write_all(b"\n\n")?;
        Ok(())
    }
}

pub fn write_to_temp() -> std::io::Result<()> {
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
