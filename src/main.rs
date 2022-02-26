mod config;
mod xstatus;

use clap::Parser;
use config::Config;

#[derive(Parser, Debug)]
#[clap(version, about = "Simple status bar for dwm", long_about = None)]
struct Cli {
    #[clap(short, long)]
    print_only: bool,

    #[clap(short, long)]
    once: bool,

    #[clap(short, long)]
    config: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let (config, config_path) = Config::new(cli.config);

    for module in &config.module {
        use std::process::Command;

        let output = Command::new("/bin/sh")
            .arg("-c")
            .args(module.cmd.split_whitespace())
            .current_dir(config_path.parent().unwrap())
            .output()
            .unwrap();

        println!("{:?}", output);
    }
}
