use crate::{EncryptionScheme, Entry};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{cmp, error::Error, fs, io::prelude::*, str::FromStr};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Notebook {
    pub file: String,
    pub dt_format: String,
    #[serde(skip)]
    entries: Vec<Entry>,
    sentiment: bool,
    encryption: Option<EncryptionScheme>,
}

impl Notebook {
    pub fn new() -> Notebook {
        Notebook {
            file: String::new(),
            dt_format: String::new(),
            entries: vec![],
            sentiment: true,
            encryption: None,
        }
    }

    pub fn write_entry(&self, entry: &Entry) -> Result<(), Box<dyn Error>> {
        let mut file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.file)
            .context(format!("unable to open or create '{}'", self.file))?;

        file.write_all(format!("{}", entry).as_bytes())
            .context(format!("unable to write to '{}'", self.file))?;
        Ok(())
    }

    pub fn read_entries(&mut self) -> Result<&mut Self, Box<dyn Error>> {
        let file =
            fs::read_to_string(&self.file).context(format!("unable to open '{}'", self.file))?;
        for e in file.split_terminator("¶\n") {
            self.entries
                .push(Entry::from_str(&e).context(format!("could not read line '{}'", e))?);
        }
        Ok(self)
    }

    pub fn read_entry<W: Write>(&self, n: &usize, mut stdout: W) -> Result<(), Box<dyn Error>> {
        let i = &self.entries.get(*n);

        match i {
            Some(e) => write!(stdout, "{}", e).context("unable to display entry")?,
            None => writeln!(stdout, "No such entry.")?,
        }

        Ok(())
    }

    pub fn list_entries<W: Write>(
        &self,
        n: &usize,
        mut stdout: W,
        l_verbose: u64,
    ) -> Result<(), Box<dyn Error>> {
        // Iterates over last n elements of entries
        // Prints timestamp numbered by enumerate
        // TODO: Indexing starts from zero, possibly change to 1?

        let i = cmp::min(self.entries.len(), *n);

        if l_verbose > 0 {
            for e in self.entries.iter().enumerate().skip(self.entries.len() - i) {
                let substr = &e.1.text[..cmp::min(30, e.1.text.len())];
                writeln!(stdout, "{}. {}... | {}", e.0, substr, e.1.timestamp)
                    .context("unable to parse entry")?;
            }
        } else {
            for e in self.entries.iter().enumerate().skip(self.entries.len() - i) {
                writeln!(stdout, "{}. {}", e.0, e.1.timestamp).context("unable to parse entry")?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test_notebook {
    use super::*;

    fn create_notebook() -> Notebook {
        let mut nb = Notebook::new();
        nb.file = "data/test.md".into();
        nb.dt_format = "%A %e %B, %Y - %H:%M".into();
        nb.read_entries().expect("Error reading entries.");
        nb
    }

    #[test]
    fn test_list_one_entry() {
        let mut stdout = vec![];
        let nb = create_notebook();
        nb.list_entries(&1, &mut stdout, 0).unwrap();
        assert_eq!(stdout, b"3. Thursday 13 May, 2021 - 22:17\n");
    }

    #[test]
    fn test_list_one_entry_verbose() {
        let mut stdout = vec![];
        let nb = create_notebook();
        nb.list_entries(&1, &mut stdout, 1).unwrap();
        assert_eq!(
            stdout,
            b"3. A terrible misfortune has happ... | Thursday 13 May, 2021 - 22:17\n"
        )
    }

    #[test]
    fn test_read_first_entry() {
        let mut stdout = vec![];
        let nb = create_notebook();
        nb.read_entry(&0, &mut stdout).unwrap();
        assert!(stdout.starts_with("### Friday 20 November, 2020 - 20:16".as_bytes()));
        assert!(stdout.ends_with("Left out the Mutlars of course.\n\n¶\n".as_bytes()));
    }

    #[test]
    fn test_outside_upper_bound() {
        let mut stdout = vec![];
        let nb = create_notebook();
        nb.read_entry(&4, &mut stdout).unwrap();
        assert_eq!(stdout, b"No such entry.\n");
    }

    #[test]
    fn test_inside_upper_bound() {
        let mut stdout = vec![];
        let nb = create_notebook();
        nb.read_entry(&3, &mut stdout).unwrap();
        assert!(stdout.starts_with("### Thursday 13 May, 2021 - 22:17".as_bytes()));
        assert!(stdout.ends_with("this seems an act of treachery.\n\n¶\n".as_bytes()));
    }
}
