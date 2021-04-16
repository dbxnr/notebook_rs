use crate::{EncryptionScheme, Entry};
use serde::{Deserialize, Serialize};
use std::{error::Error, fs, io::prelude::*, str::FromStr};

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
            .open(&self.file)?;

        file.write_all(format!("{}", entry).as_bytes())?;
        Ok(())
    }

    pub fn read_entries(&mut self) -> Result<&mut Self, Box<dyn Error>> {
        let file = fs::read_to_string(&self.file).expect("Error reading file");
        for e in file.split_terminator("¶\n") {
            self.entries.push(Entry::from_str(&e).unwrap());
        }
        Ok(self)
    }

    pub fn read_entry<W: Write>(&self, n: &usize, mut stdout: W) -> Result<(), Box<dyn Error>> {
        write!(stdout, "{}", &self.entries[*n])?;

        Ok(())
    }

    pub fn list_entries<W: Write>(&self, n: &usize, mut stdout: W) -> Result<(), Box<dyn Error>> {
        // Iterates over last n elements of entries
        // Prints timestamp numbered by enumerate
        // TODO: Indexing starts from zero, possibly change to 1?

        let mut i = self.entries.len();
        if *n < i {
            i = *n;
        }

        for e in self.entries.iter().enumerate().skip(self.entries.len() - i) {
            writeln!(stdout, "{}. {}", e.0, e.1.timestamp)?;
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
        nb.list_entries(&1, &mut stdout).unwrap();
        assert_eq!(stdout, b"3. Saturday 13 April, 1893 - 22:17\n");
    }

    #[test]
    fn test_read_first_entry() {
        let mut stdout = vec![];
        let nb = create_notebook();
        nb.read_entry(&0, &mut stdout).unwrap();
        assert!(stdout.starts_with("### Sunday 20 November, 1892 - 20:16".as_bytes()));
        assert!(stdout.ends_with("Left out the Mutlars of course.\n\n¶\n".as_bytes()));
    }
}
