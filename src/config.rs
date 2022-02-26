use serde::Deserialize;

use std::fs;
use std::path::{Path, PathBuf};

use crate::Cmd;

const DEFAULT_CONF_FILENAME: &str = "simbar.toml";

#[derive(Debug, Deserialize)]
pub struct Config {
    pub delimiter: Option<String>,
    pub padding: Option<bool>,
    pub module: Vec<Module>,
}

#[derive(Debug, Deserialize)]
pub struct Module {
    pub cmd: Cmd,
    pub repeat: Option<usize>,
    pub fg: Option<String>,
    pub bg: Option<String>,
}

impl Config {
    pub fn new(config_path: Option<impl AsRef<Path>>) -> (Self, PathBuf) {
        let config_path = match config_path {
            Some(config_path) => config_path.as_ref().to_path_buf(),
            None => {
                let xdg_dirs = xdg::BaseDirectories::with_prefix("simbar").unwrap();
                xdg_dirs
                    .find_config_file(DEFAULT_CONF_FILENAME)
                    .unwrap_or_else(|| {
                        panic!(
                            "No configuration file found, searched for: {}",
                            DEFAULT_CONF_FILENAME
                        );
                    })
            }
        };

        assert!(config_path.is_file(), "No file at: {:?}", config_path);

        let config_str = fs::read_to_string(config_path.clone()).unwrap();
        (
            toml::from_str(&config_str).unwrap(),
            fs::canonicalize(config_path).unwrap(),
        )
    }
}
