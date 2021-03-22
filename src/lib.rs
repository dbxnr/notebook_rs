use chrono::prelude::Local;
use gag::Gag;
use serde::{Deserialize, Serialize};
use std::{
    env::{temp_dir, var},
    error::Error,
    fmt, fs,
    io::prelude::*,
    process::Command,
};
use vader_sentiment::SentimentIntensityAnalyzer;

pub mod argparse;
pub mod config;

#[derive(Clone, Debug)]
pub enum Args {
    New(Journal, Entry),
}

#[derive(Clone, Debug)]
struct Sentiment {
    compound: f64,
    icon: String,
}

impl Sentiment {
    fn new(compound: f64) -> Sentiment {
        let icon = match compound {
            c if c <= -0.7 => "Awful",
            c if c <= -0.2 => "Bad",
            c if c <= 0.2 => "Neutral",
            c if c <= 0.7 => "Good",
            c if c <= 1.0 => "Great",
            _ => "Problemo",
        };

        Sentiment {
            compound,
            icon: icon.to_owned(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Entry {
    text: String,
    timestamp: String,
    sentiment: Sentiment,
}

impl Entry {
    fn new(text: String, dt_fmt: &String) -> Entry {
        let score = Entry::calculate_sentiment(&text);
        let sentiment = Sentiment::new(score);
        Entry {
            text,
            timestamp: Local::now().format(&dt_fmt).to_string(),
            sentiment,
        }
    }

    fn calculate_sentiment(text: &String) -> f64 {
        // TODO: Use pos/neg/neu as colour space coordinates
        let _print_gag = Gag::stdout().unwrap();
        let analyzer = SentimentIntensityAnalyzer::new();
        let scores = analyzer.polarity_scores(&text);

        *scores.get("compound").unwrap()
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\nMood: {}\n\n{}\n\n",
            self.timestamp, self.sentiment.icon, self.text
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Journal {
    name: String,
    file: String,
    dt_format: String,
    #[serde(skip)]
    entries: Vec<Entry>,
    encryption: Option<EncryptionScheme>,
    features: Features,
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
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct EncryptionScheme {
    cipher: bool,
    hash: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Features {
    sentiment: bool,
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
