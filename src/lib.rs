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

        let entry: String = format!(
            "{}\n{}\n\n",
            self.timestamp.to_string(),
            self.text.as_ref().unwrap()
        );

        file.write_all(entry.as_bytes())?;
        Ok(())
    }
}

pub fn write_to_temp() -> String {
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

    text
}
