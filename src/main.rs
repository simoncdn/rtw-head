use std::process;

use clap::Parser;
use rtw_head::{run, cli};

fn main() {
    let cli = cli::Cli::parse();
    if let Err(e) = run(cli) {
        eprintln!("Error: {}", e);
        process::exit(1)
    };
}
