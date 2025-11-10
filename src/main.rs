use clap::Parser;
use rtw_head::{run, cli};

fn main() {
    let cli = cli::Cli::parse();
    run(cli);
}
