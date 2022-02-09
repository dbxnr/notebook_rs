<a href="https://crates.io/crates/notebook_rs" alt="crates.io">
  <img src="https://img.shields.io/crates/v/notebook_rs" /></a>


## `notebook_rs`

CLI tool for maintaining plaintext notebooks, formatted as Markdown by default.

Still very much a work in progress.

On first use, it will create a config file in the OS specific config directory (e.g. `~/.config/notebook_rs/`). The default notebook location is `$HOME/Documents`.

Performs a fairly simple sentiment analysis on the text by default, using a Rust implementation of the VADER tool.

### Commands
- `nb -h` Summary of commands available 
- `nb -n` Opens $EDITOR for inputting text
- `nb -n <text>` Parse entry text from the commandline
- `nb -l <n>` List *n* most recent entries, use with `-v` for extra output
- `nb -r <n>` Display entry *n*
- `nb -e <n>` Edit entry *n* in system editor
- `nb -d <n>` Delete entry *n*
- `nb -s <pattern>` Search for pattern in entries

### Config settings
- `file` Path to the notebook
- `dt_format` [Time formatting syntax](https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html)
- `sentiment` Unimplemented
- `encryption` Unimplemented


### Planned features
- [ ] Search functionality
  - [x] Full-text search with regex
  - [ ] Search by date range
- [ ] Parsing tags from text
- [ ] File encryption
- [x] Editing entries
- [x] Deleting entries
- [ ] Alternative TUI
