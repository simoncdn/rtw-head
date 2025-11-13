pub mod cli;

use std::{
    error::Error,
    fs::{self},
    io::{self, BufRead, BufReader, StdoutLock, Write},
    path::Path,
};

use cli::Cli;

pub fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    let should_print_names = (cli.is_verbose || cli.files.len() > 1) && !cli.is_quiet;

    for (i, file) in cli.files.iter().enumerate() {
        print_file(&file, should_print_names, cli.lines_number)?;

        if i < cli.files.len() - 1 {
            println!();
        }
    }

    Ok(())
}

fn print_file(
    file_path: &str,
    with_file_name: bool,
    lines_number: isize,
) -> Result<(), Box<dyn Error>> {
    let path = Path::new(&file_path);
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let lines_to_print = if lines_number < 0 {
        let total_lines = count_lines(&file_path)?;
        total_lines.saturating_sub(lines_number.unsigned_abs())
    } else {
        lines_number.unsigned_abs()
    };

    if with_file_name {
        print_file_name(path, &mut handle)?;
    }

    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines().take(lines_to_print) {
        let line = line?;
        writeln!(handle, "{}", line)?
    }

    Ok(())
}

fn count_lines(file_path: &str) -> Result<usize, Box<dyn Error>> {
    let file = fs::File::open(file_path)?;
    let reader = BufReader::new(&file);
    Ok(reader.lines().count())
}

fn print_file_name(path: &Path, handle: &mut StdoutLock) -> Result<(), Box<dyn Error>> {
    let file_name = path
        .file_name()
        .ok_or("Path has no file name")?
        .to_str()
        .ok_or("File name contains invalid UTF-8")?;

    writeln!(handle, "==> {} <==", file_name)?;

    Ok(())
}
