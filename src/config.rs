use crate::{EncryptionScheme, Feature, Journal};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
struct JournalCfg {
    journals: HashMap<String, Journal>,
}

impl std::default::Default for JournalCfg {
    fn default() -> Self {
        let mut j = HashMap::new();
        j.insert(
            "default".to_string(),
            Journal {
                file: "_test.txt".into(),
                dt_format: "%A %e %B, %Y - %H:%M".into(),
                entries: vec![],
                features: vec![Feature::Sentiment],
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
        .expect("Error parsing config - does journal exist?")
        .to_owned();

    Ok(journal_cfg.to_owned())
}
