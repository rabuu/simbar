mod bar;
mod config;
mod x11_interface;

use std::thread;
use std::time::Duration;

use clap::Parser;

use bar::Bar;
use config::Config;

#[derive(Parser, Debug)]
#[clap(version, about = "Simple status bar for dwm", long_about = None)]
struct Cli {
    #[clap(short, long, help = "Only print the bar to STDOUT")]
    print: bool,

    #[clap(short, long, help = "Do not loop and run only once")]
    once: bool,

    #[clap(short, long, help = "Pass a config file")]
    config: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let mut bar = Bar::new();

    let mut sec = 0;
    loop {
        match Config::new(cli.config.as_ref()) {
            Ok(config) => bar.regenerate(config, sec),
            Err(e) => eprintln!("Invalid config: {}", e),
        }

        if cli.print {
            println!("{}", bar.bar);
        } else {
            bar.set_status();
        }

        if cli.once {
            break;
        }

        thread::sleep(Duration::from_secs(1));
        sec = (sec + 1) % usize::max_value();
    }
}
