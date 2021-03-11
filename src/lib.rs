use chrono::prelude::{DateTime, Local};
use std::{
    env::{temp_dir, var},
    error::Error,
    fmt, fs,
    io::prelude::*,
    process::Command,
};
use vader_sentiment::SentimentIntensityAnalyzer;

pub mod argparse;

#[derive(Clone, Debug)]
pub enum Args {
    New(NewEntry),
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
            compound: compound,
            icon: icon.to_owned(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct NewEntry {
    text: Option<String>,
    timestamp: DateTime<Local>,
    sentiment: Sentiment,
}

impl NewEntry {
    fn new(text: Option<String>, timestamp: DateTime<Local>) -> NewEntry {
        let score = NewEntry::calculate_sentiment(&text);
        let sentiment = Sentiment::new(score);
        NewEntry {
            text: text,
            timestamp: timestamp,
            sentiment: sentiment,
        }
    }

    fn calculate_sentiment(text: &Option<String>) -> f64 {
        // TODO: Write macro to silence this function
        // TODO: Use pos/neg/neu as colour space coordinates
        let analyzer = SentimentIntensityAnalyzer::new();
        let scores = analyzer.polarity_scores(text.as_ref().unwrap());

        *scores.get("compound").unwrap()
    }
}

impl fmt::Display for NewEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dt_format = String::from("%A %e %B, %Y - %H:%M");

        write!(
            f,
            "{}\nMood: {}\n\n{}\n\n",
            self.timestamp.format(&dt_format),
            self.sentiment.icon,
            self.text.as_ref().unwrap()
        )
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

        file.write_all(format!("{}", entry).as_bytes())?;
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
