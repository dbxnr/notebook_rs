use crate::Notebook;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
struct NotebookCfg {
    notebooks: HashMap<String, Notebook>,
}

impl std::default::Default for NotebookCfg {
    fn default() -> Self {
        let mut j = HashMap::new();
        let mut nb = Notebook::new();
        nb.file = "notebook.md".into();
        nb.dt_format = "%A %e %B, %Y - %H:%M".into();
        j.insert("default".to_string(), nb);
        Self { notebooks: j }
    }
}

pub fn read_config(notebook: Option<&str>) -> Result<Notebook, confy::ConfyError> {
    let notebook_name = notebook.unwrap_or("default");
    let notebook_cfg: NotebookCfg = confy::load("notebook").expect("Error reading config");

    let notebook_cfg = &notebook_cfg
        .notebooks
        .get(notebook_name)
        .expect("Error parsing config - does notebook exist?")
        .to_owned();

    Ok(notebook_cfg.to_owned())
}
