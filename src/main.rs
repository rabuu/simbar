mod config;
mod xstatus;

use clap::Parser;
use config::Config;
use xstatus::XStatus;

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
    let config = Config::new(cli.config);
}
