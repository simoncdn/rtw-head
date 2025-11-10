pub mod cli;

use cli::Cli;

pub fn run(cli: Cli) {
    println!("file: {}", cli.file_path);
}

#[cfg(test)]
mod tests { }
