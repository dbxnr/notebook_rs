use crate::notebook::Notebook;

use entry::Entry;
use serde::{Deserialize, Serialize};
use std::{env, fmt, fs, io::prelude::*, process::Command};

pub mod argparse;
pub mod config;
pub mod entry;
pub mod notebook;

#[derive(Clone, Debug)]
pub enum Args<'a> {
    New(&'a Notebook, Entry),
    List(&'a Notebook, usize, u64),
    Read(&'a Notebook, usize),
}

#[derive(Clone, Debug)]
struct Sentiment {
    compound: f64,
    icon: String,
}

impl Sentiment {
    fn new(compound: f64) -> Sentiment {
        let icon = match compound {
            c if c <= -0.7 => "😿",
            c if c <= -0.2 => "😾",
            c if c <= 0.2 => "🐱",
            c if c <= 0.7 => "😺",
            c if c <= 1.0 => "😸",
            _ => "Problemo",
        };

        Sentiment {
            compound,
            icon: icon.to_owned(),
        }
    }
}

impl fmt::Display for Sentiment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.3} ≅ {}", self.compound, self.icon)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct EncryptionScheme {
    cipher: bool,
    hash: bool,
    salt: bool,
}

pub fn text_from_editor() -> Option<String> {
    let editor = env::var("EDITOR").expect("EDITOR environment variable is missing.");
    let mut file_path = env::temp_dir();
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

    if text.is_empty() {
        None
    } else {
        Some(text)
    }
}

#[cfg(test)]
mod test_util {
    use super::*;

    #[test]
    fn test_missing_editor_variable() {
        env::remove_var("EDITOR");
        let result = std::panic::catch_unwind(|| text_from_editor());
        assert!(result.is_err());
    }
}
