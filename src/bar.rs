use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

use regex::Regex;

use crate::config::{Config, Module};
use crate::x11_interface::X11Interface;

const DEFAULT_DELIMITER: &str = " | ";

/// The status bar
///
/// # Fields
/// - `bar`: The contents of the bar
/// - `cache`: A cache to store values that don't need to be regenerated all the time
/// - `xinterface`: An interface to communicate with the X11 Server
pub struct Bar {
    pub bar: String,
    cache: HashMap<String, String>,
    xinterface: X11Interface,
}

impl Bar {
    /// Constructor for `Bar`
    pub fn new() -> Self {
        Bar {
            bar: String::new(),
            cache: HashMap::new(),
            xinterface: X11Interface::new(),
        }
    }

    /// Regenerate the bar
    ///
    /// # Parameters
    /// - `(config, config_path)`: The bar configuration and the path where it's located
    /// - `sec`: The current second
    pub fn regenerate(&mut self, (config, config_path): (Config, PathBuf), sec: usize) {
        // clear the bar
        self.bar.clear();

        // set delimiter as specfied in config
        let del = match &config.delimiter {
            Some(ref del) => del.as_str(),
            None => DEFAULT_DELIMITER,
        };

        // loop over all modules from config
        for module in &config.module {
            let repeat = module.repeat.unwrap_or(0);

            // evaluate module value and cache it
            if (repeat >= 1 && sec % repeat == 0) || !self.cache.contains_key(&module.name) {
                let args = shell_words::split(&format!("'{}'", module.cmd))
                    .expect("Command could not be parsed");

                let output = Command::new("/bin/sh")
                    .arg("-c")
                    .args(args)
                    .current_dir(config_path.parent().unwrap())
                    .output()
                    .unwrap();

                let output = if output.stderr.is_empty() {
                    if let Ok(output) = String::from_utf8(output.stdout) {
                        output
                    } else {
                        self.cache.remove(&module.name);
                        continue;
                    }
                } else {
                    self.cache.remove(&module.name);
                    continue;
                };

                self.cache.insert(module.name.clone(), output);
            }

            // add module value to bar
            self.push_module(module);

            // add delimiter to bar
            self.bar.push_str(del);
        }

        // remove trailing delimiter
        if let Some(stripped) = self.bar.strip_suffix(del) {
            self.bar = stripped.to_string();
        }

        // add padding to bar
        if config.padding.unwrap_or(false) {
            self.bar.insert(0, ' ');
            self.bar.push(' ');
        }
    }

    fn push_module(&mut self, module: &Module) {
        // specify how a valid hex colorcode has to look like
        let hexcol_regex = Regex::new(r"^#[A-Fa-f0-9]{6}$").unwrap();

        // set foreground color
        let fg = if let Some(fg) = &module.fg {
            let col = match fg.trim() {
                "xresources" => self
                    .xinterface
                    .get_xresource(module.name.clone() + ".fg")
                    .unwrap_or_default(),
                literal => literal.to_string(),
            };

            if hexcol_regex.is_match(&col) {
                Some(col)
            } else {
                None
            }
        } else {
            None
        };

        // set background color
        let bg = if let Some(bg) = &module.bg {
            let col = match bg.trim() {
                "xresources" => self
                    .xinterface
                    .get_xresource(module.name.clone() + ".bg")
                    .unwrap_or_default(),
                literal => literal.to_string(),
            };

            if hexcol_regex.is_match(&col) {
                Some(col)
            } else {
                None
            }
        } else {
            None
        };

        // add fg to bar
        if let Some(ref fg) = fg {
            self.bar.push_str(&format!("^c{}^", fg));
        }

        // add bg to bar
        if let Some(ref bg) = bg {
            self.bar.push_str(&format!("^b{}^", bg));
        }

        // add prefix to bar
        if let Some(ref pre) = module.prefix {
            self.bar.push_str(pre);
        }

        // add module value itself to bar
        let value = self.cache.get(&module.name).unwrap().trim();
        self.bar.push_str(value);

        // add suffix to bar
        if let Some(ref suf) = module.suffix {
            self.bar.push_str(suf);
        }

        // close color tag
        if fg.is_some() || bg.is_some() {
            self.bar.push_str("^d^");
        }
    }

    /// Set X11 status according to bar
    pub fn set_status(&self) {
        self.xinterface.set_status(&self.bar);
    }
}
