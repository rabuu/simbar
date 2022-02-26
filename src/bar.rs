use std::{collections::HashMap, path::PathBuf, process::Command};

use regex::Regex;

use crate::{config::Config, Cmd};

const DEFAULT_DELIMITER: &str = " | ";

pub struct Bar {
    cache: HashMap<Cmd, String>,
}

impl Bar {
    pub fn new() -> Self {
        Bar {
            cache: HashMap::new(),
        }
    }

    pub fn generate_bar(&mut self, (config, config_path): (Config, PathBuf), sec: usize) -> String {
        let mut bar = String::new();

        let len = config.module.len();
        for (i, module) in config.module.iter().enumerate() {
            let repeat = module.repeat.unwrap_or(0);

            let mut should_cache = false;
            if repeat >= 1 && sec % repeat == 0 {
                should_cache = true;
            } else {
                if !self.cache.contains_key(&module.cmd) {
                    should_cache = true;
                }
            }

            if should_cache {
                let output = Command::new("/bin/sh")
                    .arg("-c")
                    .args(module.cmd.split_whitespace())
                    .current_dir(config_path.parent().unwrap())
                    .output()
                    .unwrap();
                assert!(output.stderr.is_empty(), "Cmd failed: {:?}", output);
                let output = String::from_utf8(output.stdout).unwrap();
                self.cache.insert(module.cmd.clone(), output);
            }

            let value = self.cache.get(&module.cmd).unwrap().trim();
            bar.push_str(&wrap_into_colors(
                value,
                module.fg.as_ref(),
                module.bg.as_ref(),
            ));

            if i < len - 1 {
                let del = match &config.delimiter {
                    Some(ref del) => del.as_str(),
                    None => DEFAULT_DELIMITER,
                };

                bar.push_str(del);
            }
        }

        if config.padding.unwrap_or(false) {
            bar.insert_str(0, " ");
            bar.push(' ');
        }

        bar
    }
}

fn wrap_into_colors(s: &str, fg: Option<&String>, bg: Option<&String>) -> String {
    let mut wrapped = String::new();

    let hexcol_regex = Regex::new(r"^#[A-Fa-f0-9]{6}$").unwrap();
    if let Some(fg) = fg {
        let col = if fg.starts_with("xres:") {
            eprintln!("xres is currently unimplemented");
            "ERROR"
        } else {
            fg.as_str()
        };

        if hexcol_regex.is_match(col) {
            wrapped.push_str(&format!("^c{}^", col));
        } else {
            eprintln!("Invalid color: {}", col);
        }
    }

    if let Some(bg) = bg {
        let col = if bg.starts_with("xres:") {
            eprintln!("xres is currently unimplemented");
            "ERROR"
        } else {
            bg.as_str()
        };

        if hexcol_regex.is_match(col) {
            wrapped.push_str(&format!("^c{}^", col));
        } else {
            eprintln!("Invalid color: {}", col);
        }
    }

    wrapped.push_str(s);

    if fg.is_some() || bg.is_some() {
        wrapped.push_str("^d^")
    }

    wrapped
}
