use chrono::prelude::Local;
use gag::Gag;
use std::{fmt, str::FromStr, string::ParseError};
use vader_sentiment::SentimentIntensityAnalyzer;

use crate::Sentiment;

#[derive(Clone, Debug)]
pub struct Entry {
    pub text: String,
    pub timestamp: String,
    sentiment: Sentiment,
}

impl Entry {
    pub fn new(text: String, dt_fmt: &str) -> Entry {
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
        let header: Vec<&str> = e[0].trim().split('\n').collect();
        let compound: f64 = header[1].split('≅').collect::<Vec<&str>>()[0][5..]
            .trim()
            .parse()
            .unwrap();
        Ok(Entry {
            text: e[1].trim().into(),
            timestamp: header[0].split_at(4).1.into(),
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

#[cfg(test)]
mod test_entry {
    use super::*;

    #[test]
    fn test_entry_text() {
        let e = Entry::new("Testing this entry".into(), "%A %e %B, %Y - %H:%M");
        assert_eq!(e.text, "Testing this entry");
    }

    #[test]
    fn test_timestamp_is_now() {
        let e = Entry::new("Testing the timestamp".into(), "%A %e %B, %Y - %H:%M");
        assert_eq!(
            e.timestamp,
            Local::now().format("%A %e %B, %Y - %H:%M").to_string()
        );
    }

    #[test]
    fn test_sentiment_positive() {
        let e = Entry::new(
            "This is amazing and should have a really high and awesome compound sentiment!".into(),
            "%A %e %B, %Y - %H:%M",
        );
        assert_eq!(e.sentiment.compound, 0.86732358124633);
    }

    #[test]
    fn test_sentiment_negative() {
        let e = Entry::new(
            "This is awful and should have a really low and terrible compound sentiment!".into(),
            "%A %e %B, %Y - %H:%M",
        );
        assert_eq!(e.sentiment.compound, -0.8157728811846393);
    }
}
