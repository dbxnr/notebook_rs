use chrono::prelude::{DateTime, Local};
use std::{
    env::{temp_dir, var},
    error::Error,
    fs,
    io::prelude::*,
    process::Command,
};

pub mod argparse;

#[derive(Clone, Debug)]
pub enum Args {
    New(NewEntry),
}

#[derive(Clone, Debug)]
pub struct NewEntry {
    text: Option<String>,
    timestamp: DateTime<Local>,
}

impl NewEntry {
    fn new(text: Option<String>, timestamp: DateTime<Local>) -> NewEntry {
        NewEntry {
            text: text,
            timestamp: timestamp,
        }
    }
}

#[derive(Debug)]
pub struct Journal {
    pub cmd: Args,
    filename: Option<String>,
    dt_format: String,
}

impl Journal {
    fn new(cmd: &Args, filename: Option<String>) -> Journal {
        Journal {
            cmd: cmd.to_owned(),
            filename: filename,
            dt_format: String::from("%A %e %B, %Y - %H:%M"),
        }
    }

    pub fn write_entry(&self, entry: &NewEntry) -> Result<(), Box<dyn Error>> {
        let mut file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open("_test.txt")?;

        let entry: String = format!(
            "{}\n{}\n\n",
            entry.timestamp.format(&self.dt_format),
            entry.text.as_ref().unwrap()
        );

        file.write_all(entry.as_bytes())?;
        Ok(())
    }
}

pub fn text_from_editor() -> String {
    let editor = var("EDITOR").unwrap();
    let mut file_path = temp_dir();
    file_path.push("editable");
    fs::File::create(&file_path).expect("Could not create file.");

    Command::new(editor)
        .arg(&file_path)
        .status()
        .expect("Something went wrong with the editor.");

    let mut text = String::new();
    fs::File::open(&file_path)
        .expect("Couldn't open temp file.")
        .read_to_string(&mut text)
        .expect("Couldn't load file to string.");

    fs::remove_file(file_path).expect("Couldn't remove temp file.");

    // TODO: Check for empty string
    text
}
