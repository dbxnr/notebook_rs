use crate::{create_temp_file, get_user_confirm, text_from_editor, Args, EncryptionScheme, Entry};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{cmp, error::Error, fs, io, io::prelude::*, str::FromStr};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Notebook {
    pub file: String,
    pub dt_format: String,
    #[serde(skip)]
    entries: Vec<Entry>,
    sentiment: bool,
    encryption: Option<EncryptionScheme>,
}

impl Default for Notebook {
    fn default() -> Self {
        Self::new()
    }
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

    pub fn write_entry(
        &self,
        entry: &Entry,
        path: Option<&String>,
    ) -> Result<&Self, Box<dyn Error>> {
        let mut file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(path.unwrap_or(&self.file))
            .context(format!("unable to open or create '{}'", self.file))?;

        file.write_all(format!("{}", entry).as_bytes())
            .context(format!("unable to write to '{}'", self.file))?;
        Ok(self)
    }

    pub fn new_entry(&mut self, entry: Entry) -> Result<&Self, Box<dyn Error>> {
        self.entries.push(entry);

        Ok(self)
    }

    pub fn write_all_entries(&self) -> Result<&Self, Box<dyn Error>> {
        // Write all entries to tmp file, overwrite notebook, remove temp file.
        let file_path = create_temp_file(None);

        for e in &self.entries {
            self.write_entry(e, Some(&file_path))?;
        }
        fs::copy(&file_path, &self.file)
            .context(format!("unable to copy file to '{}'", &self.file))?;
        fs::remove_file(&file_path).expect("Couldn't remove temp file.");
        Ok(self)
    }

    pub fn populate_notebook(mut self) -> Result<Self, Box<dyn Error>> {
        let file =
            fs::read_to_string(&self.file).context(format!("unable to open '{}'", self.file))?;
        for e in file.split_terminator("¶\n") {
            self.entries.push(
                Entry::from_str(e).context(format!("could not read line '{}'", e.to_owned()))?,
            );
        }
        Ok(self)
    }

    pub fn read_entry<W: Write>(&self, n: &usize, mut stdout: W) -> Result<&Self, Box<dyn Error>> {
        let i = &self.entries.get(*n);

        match i {
            Some(e) => write!(stdout, "{}", e).context("unable to display entry")?,
            None => writeln!(stdout, "No such entry.")?,
        }

        Ok(self)
    }

    pub fn list_entries<W: Write>(
        &self,
        n: &usize,
        mut stdout: W,
        l_verbose: u64,
    ) -> Result<&Self, Box<dyn Error>> {
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

        Ok(self)
    }

    pub fn edit_entry(&mut self, n: usize) -> Result<&Self, Box<dyn Error>> {
        let e = &mut self
            .entries
            .get_mut(n)
            .expect("Unable to read entry, may not exist.");

        let temp_file = create_temp_file(Some("notebook_entry"));
        fs::write(&temp_file, &e.text).expect("Error writing to temp file");
        let edited_entry = text_from_editor(Some(temp_file)).unwrap();

        e.replace_text(&edited_entry);

        Ok(self)
    }

    pub fn delete_entry(&mut self, n: usize, conf_req: bool) -> Result<&Self, Box<dyn Error>> {
        if conf_req
            && get_user_confirm(
                &mut io::stdin().lock(),
                format!("Confirm delete entry {n}?"),
            )
        {
            self.entries.remove(n);
            println!("Deleted entry {n}");
        } else {
            self.entries.remove(n);
        }

        Ok(self)
    }

    pub fn run_command(mut self, cmd: Args) -> Result<Self, Box<dyn Error>> {
        match cmd {
            Args::New(e) => self.new_entry(e),
            Args::List(ref n, l) => self.list_entries(n, &mut io::stdout(), l),
            Args::Read(ref n) => self.read_entry(n, &mut io::stdout()),
            Args::Edit(n) => self.edit_entry(n),
            Args::Delete(n, conf) => self.delete_entry(n, conf),
            Args::Unimplemented() => panic!("Not implemented"),
        }
        .expect("Error matching command");

        Ok(self)
    }
}

#[cfg(test)]
mod test_notebook {
    use super::*;

    fn create_notebook() -> Notebook {
        let mut nb = Notebook::new();
        nb.file = "data/test.md".into();
        nb.dt_format = "%A %e %B, %Y - %H:%M".into();
        let nb = nb.populate_notebook().expect("Error reading entries.");
        nb
    }

    #[test]
    fn test_populate_notebook() {
        let mut nb = Notebook::new();
        nb.file = "data/test.md".into();
        nb.dt_format = "%A %e %B, %Y - %H:%M".into();
        let nb = nb.populate_notebook().expect("Error reading entries.");
        assert_eq!(nb.entries.len(), 4);
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

    #[test]
    fn test_delete_entry() {
        let mut stdout = vec![];
        let mut nb = create_notebook();
        assert_eq!(nb.entries.len(), 4);
        nb.read_entry(&0, &mut stdout).unwrap();
        nb.delete_entry(2, false).unwrap();
        assert_eq!(nb.entries.len(), 3);
    }
}
