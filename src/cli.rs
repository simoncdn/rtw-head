use clap::Parser;

#[derive(Parser)]
#[command(name = "rtw-head")]
#[command(version = "0.1.0")]
#[command(about = "A head clone written in Rust")]
pub struct Cli {
    /// The file to display
    pub file_path: String,
}
