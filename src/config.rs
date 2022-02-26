use serde::Deserialize;

use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

const DEFAULT_CONF_FILENAME: &str = "simbar.toml";

#[derive(Debug, Deserialize)]
pub struct Config {
    pub delimiter: String,
    pub padding: bool,
    pub module: Vec<Module>,
}

#[derive(Debug, Deserialize)]
pub struct Module {
    pub cmd: String,
    pub repeat: Option<usize>,
    pub fg: Option<String>,
    pub bg: Option<String>,
}

impl Config {
    pub fn new(config_path: Option<String>) -> (Self, PathBuf) {
        let config_path = match config_path {
            Some(config_path) => PathBuf::from_str(&config_path).expect("No valid path supplied"),
            None => {
                let xdg_dirs = xdg::BaseDirectories::with_prefix("simbar").unwrap();
                xdg_dirs
                    .find_config_file(DEFAULT_CONF_FILENAME)
                    .unwrap_or_else(|| {
                        panic!(
                            "No configuration directory found, searched for: {}",
                            DEFAULT_CONF_FILENAME
                        );
                    })
            }
        };

        assert!(config_path.is_file(), "No file at: {:?}", config_path);

        let mut modules_dir = config_path.clone();
        modules_dir.pop();
        modules_dir.push("modules/");
        assert!(modules_dir.is_dir(), "No directory at: {:?}", modules_dir);

        let config_str = fs::read_to_string(config_path.clone()).unwrap();
        (
            toml::from_str(&config_str).unwrap(),
            fs::canonicalize(config_path).unwrap(),
        )
    }
}
