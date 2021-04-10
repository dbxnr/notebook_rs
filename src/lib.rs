use chrono::prelude::Local;
use gag::Gag;
use serde::{Deserialize, Serialize};
use std::{
    env::{temp_dir, var},
    error::Error,
    fmt, fs,
    io::prelude::*,
    process::Command,
    str::FromStr,
    string::ParseError,
};
use vader_sentiment::SentimentIntensityAnalyzer;

pub mod argparse;
pub mod config;

#[derive(Clone, Debug)]
pub enum Args<'a> {
    New(&'a Journal, Entry),
    List(&'a Journal),
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

#[derive(Clone, Debug)]
pub struct Entry {
    text: String,
    timestamp: String,
    sentiment: Sentiment,
}

impl Entry {
    fn new(text: String, dt_fmt: &str) -> Entry {
        let score = Entry::calculate_sentiment(&text);
        let sentiment = Sentiment::new(score);
        Entry {
            text,
            timestamp: Local::now().format(&dt_fmt).to_string(),
            sentiment,
        }
    }

    fn calculate_sentiment(text: &str) -> f64 {
        // TODO: Use pos/neg/neu as colour space coordinates
        let _print_gag = Gag::stdout().unwrap();
        let analyzer = SentimentIntensityAnalyzer::new();
        let scores = analyzer.polarity_scores(&text);

        *scores.get("compound").unwrap()
    }
}

impl FromStr for Entry {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let e: Vec<&str> = s.split("---").collect();

        // Use str::split_once when available
        // Or use regex
        let text = e[1].trim().to_string();
        let header: Vec<&str> = e[0].trim().split('\n').collect();
        let timestamp = header[0].split_at(4).1.to_string();
        let compound: f64 = header[1].split('-').collect::<Vec<&str>>()[0][5..10]
            .parse()
            .unwrap(); //.trim();
        Ok(Entry {
            text,
            timestamp,
            sentiment: { Sentiment::new(compound) },
        })
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "### {}\n#### {}\n---\n\n{}\n\n¶\n",
            self.timestamp, self.sentiment, self.text
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Journal {
    file: String,
    dt_format: String,
    #[serde(skip)]
    entries: Vec<Entry>,
    sentiment: bool,
    encryption: Option<EncryptionScheme>,
}

impl Journal {
    pub fn write_entry(&self, entry: &Entry) -> Result<(), Box<dyn Error>> {
        let mut file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.file)?;

        file.write_all(format!("{}", entry).as_bytes())?;
        Ok(())
    }

    pub fn read_entry(&self) -> Result<(), Box<dyn Error>> {
        let file = fs::read_to_string(&self.file).expect("Error reading file");
        for e in file.split_terminator("¶\n") {
            println!("{}", Entry::from_str(&e).unwrap());
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct EncryptionScheme {
    cipher: bool,
    hash: bool,
    salt: bool,
}

pub fn text_from_editor() -> Option<String> {
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

    if text.is_empty() {
        None
    } else {
        Some(text)
    }
}
