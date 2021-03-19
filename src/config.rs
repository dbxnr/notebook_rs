use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct JournalCfg {
    journals: HashMap<String, Journal>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Journal {
    name: String,
    file: String,
    dtformat: String,
    encryption: Option<EncryptionScheme>,
    features: Features,
}

#[derive(Debug, Serialize, Deserialize)]
struct EncryptionScheme {
    cipher: bool,
    hash: bool,
}

#[derive(Debug, Serialize, Deserialize)]
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

pub fn test_config() -> Result<(), confy::ConfyError> {
    let l_cfg: JournalCfg = confy::load("journal").unwrap();

    dbg!(l_cfg);
    Ok(())
}
