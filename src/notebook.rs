use crate::{create_temp_file, get_user_confirm, text_from_editor, Args, EncryptionScheme, Entry};
use ansi_term::{Colour::Red, Style};
use anyhow::{Context, Result};
use regex::Regex;
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
    #[serde(skip)]
    search_result: Vec<SearchResult>,
}

#[derive(Clone, Debug)]
struct SearchResult {
    pub pattern: Regex,
    pub entry_idx: usize,
    pub location: Vec<String>,
}

impl SearchResult {
    pub fn new(pattern: Regex, entry_idx: usize, location: Vec<String>) -> SearchResult {
        SearchResult {
            pattern,
            entry_idx,
            location,
        }
    }
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
            search_result: vec![],
        }
    }

    pub fn write_entry(&self, entry: &Entry, path: &String) -> Result<&Self, Box<dyn Error>> {
        let mut file = fs::OpenOptions::new()
            .append(true)
            .open(path)
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
        let temp_file_path = create_temp_file(None);

        for e in &self.entries {
            self.write_entry(e, &temp_file_path)
                .expect("Error writing to temp file");
        }
        fs::copy(&temp_file_path, &self.file)
            .context(format!("unable to copy file to '{}'", &self.file))?;
        fs::remove_file(&temp_file_path).expect("Couldn't remove temp file.");
        Ok(self)
    }

    /// Opens the file
    /// Reads the contents to a string
    /// Populates the Notebook instance with entries
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
                writeln!(
                    stdout,
                    "{}. {}... | {}",
                    e.0,
                    substr,
                    e.1.timestamp.format(&self.dt_format)
                )
                .context("unable to parse entry")?;
            }
        } else {
            for e in self.entries.iter().enumerate().skip(self.entries.len() - i) {
                writeln!(stdout, "{}. {}", e.0, e.1.timestamp.format(&self.dt_format))
                    .context("unable to parse entry")?;
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
            Args::Search(s) => self
                .search(s)
                .unwrap()
                .output_search_results(&mut io::stdout()),
            Args::Unimplemented() => panic!("Not implemented"),
        }
        .expect("Error matching command");

        Ok(self)
    }

    fn search(&mut self, q: String) -> Result<&Self, Box<dyn Error>> {
        // TODO: Currently case-sensitive, add flag to toggle
        let regex = Regex::new(&q).expect("Error compiling regex.");

        for (e_idx, e) in self.entries.iter().enumerate() {
            if regex.is_match(&e.text) {
                let matches = regex
                    .find_iter(&e.text)
                    .map(|digits| digits.as_str().to_owned())
                    .collect::<Vec<String>>();

                self.search_result
                    .push(SearchResult::new(regex.clone(), e_idx, matches));
            }
        }

        Ok(self)
    }

    fn output_search_results<W: Write>(&self, mut stdout: W) -> Result<&Self, Box<dyn Error>> {
        // Break string into 50 char blocks
        // Iterate over blocks
        // Print only ones with matches
        for r in &self.search_result {
            write!(
                stdout,
                "{}: {}\t",
                Style::new().bold().paint(r.entry_idx.to_string()),
                Style::new()
                    .bold()
                    .paint(self.entries[r.entry_idx].timestamp.to_string())
            )?;
            for (idx, c) in r.pattern.split(&self.entries[r.entry_idx].text).enumerate() {
                write!(stdout, "{}", c)?;

                if let Some(c) = r.location.get(idx) {
                    write!(stdout, "{}", Red.paint(c))?;
                };
            }
            writeln!(stdout)?;
        }
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
        let nb = nb.populate_notebook().expect("Error reading notebook.");
        nb
    }

    #[test]
    fn test_populate_notebook() {
        let mut nb = Notebook::new();
        nb.file = "data/test.md".into();
        nb.dt_format = "%A %e %B, %Y - %H:%M".into();
        let nb = nb.populate_notebook().expect("Error reading notebook.");
        assert_eq!(nb.entries.len(), 4);
    }

    #[test]
    fn test_populate_blank_notebook() {
        let mut nb = Notebook::new();
        nb.file = "data/empty.md".into();
        nb.dt_format = "%A %e %B, %Y - %H:%M".into();
        let nb = nb.populate_notebook().expect("Error reading notebook.");
        assert_eq!(nb.entries.len(), 0);
    }
    #[test]
    fn test_new_entry() {
        let e = Entry::new("Testing this entry".into(), "%A %e %B, %Y - %H:%M");
        let mut nb = create_notebook();
        nb.new_entry(e).unwrap();
        assert_eq!(nb.entries.len(), 5);
        assert_eq!(nb.entries[4].text, "Testing this entry");
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
    fn test_read_outside_upper_bound() {
        let mut stdout = vec![];
        let nb = create_notebook();
        nb.read_entry(&4, &mut stdout).unwrap();
        assert_eq!(stdout, b"No such entry.\n");
    }

    #[test]
    fn test_read_inside_upper_bound() {
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

    #[test]
    fn test_search_word_single_result() {
        let mut nb = create_notebook();
        nb.search("brandy".into()).unwrap();
        assert_eq!(nb.search_result.len(), 1);
    }

    #[test]
    fn test_search_phrase_single_result() {
        let mut nb = create_notebook();
        nb.search("poisoned by some lobster".into()).unwrap();
        assert_eq!(nb.search_result.len(), 1);
    }

    #[test]
    fn test_search_multiple_results() {
        let mut nb = create_notebook();
        nb.search("Lupin".into()).unwrap();
        assert_eq!(nb.search_result.len(), 3);
    }

    #[test]
    fn test_search_zero_results() {
        let mut nb = create_notebook();
        nb.search("zebra".into()).unwrap();
        assert_eq!(nb.search_result.len(), 0);
    }

    #[test]
    fn test_search_correct_location() {
        let mut nb = create_notebook();
        nb.search("Crowbillon".into()).unwrap();
        assert_eq!(nb.search_result[0].location[0], "Crowbillon");
        nb.search("’".into()).unwrap();
        assert_eq!(nb.search_result[1].location[1], "’");
    }

    #[test]
    fn test_search_output() {
        let mut stdout = vec![];
        let mut nb = create_notebook();
        nb.search("Crowbillon".into())
            .unwrap()
            .output_search_results(&mut stdout)
            .unwrap();
        assert!(&stdout.starts_with("\u{1b}[1m3\u{1b}[0m: \u{1b}[1m2021-05-13 22:17:00\u{1b}[0m\tA terrible misfortune has happened:".as_bytes()));
    }
}
