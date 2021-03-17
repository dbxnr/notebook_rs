use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct JournalCfg {
    journals: Journal,
}

#[derive(Debug, Serialize, Deserialize)]
struct Journal {
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

impl ::std::default::Default for JournalCfg {
    fn default() -> Self {
        Self {
            journals: Journal {
                file: "_test.txt".into(),
                dtformat: "this-is-not-a-key".into(),
                features: Features { sentiment: true },
                encryption: Some(EncryptionScheme {
                    cipher: false,
                    hash: false,
                }),
            },
        }
    }
}

pub fn test_config() -> Result<(), confy::ConfyError> {
    let l_cfg: JournalCfg = confy::load("journal").unwrap();

    dbg!(l_cfg);
    Ok(())
}
