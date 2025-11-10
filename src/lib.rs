pub mod cli;

use std::{
    error::Error,
    fs,
    io::{self, BufRead, BufReader, Write},
};

use cli::Cli;

pub fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    let file = fs::File::open(cli.file_path)?;

    let reader = BufReader::new(file);
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for line in reader.lines().take(cli.lines_number) {
        let line = line?;
        writeln!(handle, "{}", line)?
    }

    Ok(())
}
