use clap::Parser;

#[derive(Parser)]
#[command(name = "rtw-head")]
#[command(version = "0.1.0")]
#[command(about = "A head clone written in Rust")]
pub struct Cli {
    /// file to display
    pub file_path: String,

    /// print the first NUM lines instead of the first 10; with the leading '-', print all but the last NUM lines of each file
    #[arg(short = 'n', long = "lines", default_value_t = 10)]
    pub lines_number: usize,
}
