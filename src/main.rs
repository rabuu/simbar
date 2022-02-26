mod config;

use clap::{ArgGroup, Parser};
use config::Config;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(group(ArgGroup::new("out_mode").required(true).multiple(false)))]
#[clap(group(ArgGroup::new("freq_mode").required(true).multiple(false)))]
struct Cli {
    #[clap(short, long, group = "out_mode")]
    print: bool,
    #[clap(short, long, group = "out_mode")]
    xsetroot: bool,

    #[clap(short, long, group = "freq_mode")]
    once: bool,
    #[clap(short, long, group = "freq_mode")]
    repeat: bool,

    #[clap(short, long)]
    config: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let config = Config::new(cli.config);

    println!("{:#?}", config);
}
