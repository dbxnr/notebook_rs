use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct JournalCfg {
    journals: HashMap<String, Journal>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Journal {
    name: String,
    file: String,
    dtformat: String,
    encryption: Option<EncryptionScheme>,
    features: Features,
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

impl std::default::Default for JournalCfg {
    fn default() -> Self {
        let mut j = HashMap::new();
        j.insert(
            "default".to_string(),
            Journal {
                name: "default".into(),
                file: "_test.txt".into(),
                dtformat: "%A %e %B, %Y - %H:%M".into(),
                features: Features { sentiment: true },
                encryption: Some(EncryptionScheme {
                    cipher: false,
                    hash: false,
                }),
            },
        );
        Self { journals: j }
    }
}

pub fn read_config(journal: Option<&str>) -> Result<Journal, confy::ConfyError> {
    let journal_name = journal.unwrap_or("default");
    let journal_cfg: JournalCfg = confy::load("journal").expect("Error reading config");

    let journal_cfg = &journal_cfg
        .journals
        .get(journal_name)
        .expect("Error parsing config")
        .to_owned();

    Ok(journal_cfg.to_owned())
}
