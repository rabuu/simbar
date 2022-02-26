mod bar;
mod config;
mod xstatus;

use std::{thread, time::Duration};

use clap::Parser;

use bar::Bar;
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

type Cmd = String;

fn main() {
    let cli = Cli::parse();

    let xstatus: Option<XStatus> = match cli.print_only {
        true => None,
        false => Some(XStatus::new()),
    };

    let mut bar = Bar::new();

    let mut secs = 0;
    loop {
        let bar = bar.generate_bar(Config::new(cli.config.as_ref()), secs);

        if cli.print_only {
            println!("{}", bar);
        } else {
            xstatus.as_ref().unwrap().set_status(&bar);
        }

        if cli.once {
            break;
        }

        thread::sleep(Duration::from_secs(1));
        secs += 1;
    }
}
