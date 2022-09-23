use crate::Notebook;
use directories::{BaseDirs, UserDirs};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, fs::OpenOptions, path::PathBuf};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
struct NotebookCfg {
    notebooks: HashMap<String, Notebook>,
}

impl std::default::Default for NotebookCfg {
    fn default() -> Self {
        let nb_path = get_documents_dir().join("notebook.md");
        let mut j = HashMap::new();
        let mut nb = Notebook::new();
        nb.file = nb_path.to_str().unwrap().into();
        nb.dt_format = "%A %e %B, %Y - %H:%M".into();
        j.insert("default".to_string(), nb);
        Self { notebooks: j }
    }
}

pub fn get_documents_dir() -> PathBuf {
    // If XDG_DOCUMENT_DIR does not exist, returns XDG_HOME
    let base_dirs = BaseDirs::new().unwrap();
    let home_dir = base_dirs.home_dir();
    let user_dirs = UserDirs::new().unwrap();
    let document_dir = user_dirs.document_dir().unwrap_or(home_dir);
    PathBuf::from(document_dir)
}

pub fn read_config(notebook: Option<&str>) -> Result<Notebook, confy::ConfyError> {
    // This should return the config file
    let notebook_name = notebook.unwrap_or("default");
    let notebook_cfg: NotebookCfg = confy::load("notebook_rs").expect("Error reading config");

    let notebook_cfg = &notebook_cfg
        .notebooks
        .get(notebook_name)
        .expect("Error parsing config - does notebook exist?")
        .to_owned();

    Ok(notebook_cfg.to_owned())
}

pub fn check_create_file(path: &String) -> Result<PathBuf, Box<dyn Error>> {
    // This should return the path
    let p = PathBuf::from(path);
    OpenOptions::new().write(true).create(true).open(&p)?;
    Ok(p)
}
