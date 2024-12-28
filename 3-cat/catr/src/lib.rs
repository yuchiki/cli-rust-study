use std::error::Error;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Cat in Rust
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Arg {

    /// Name of files
    #[arg(default_values_t = ["-".to_string()])]
    files: Vec<String>,

    /// If true, number all output lines
    #[arg(short, long, conflicts_with = "number_nonblank")]
    number: bool,

    /// If true, number non-blank output lines
    #[arg(short = 'b', long)]
    number_nonblank: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Arg> {
    let args = Arg::parse();
    Ok(args)
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new( File::open(filename)?))),
    }
}

pub fn run(arg : Arg) -> MyResult<()> {
    for filename in arg.files {
        match open(&filename) {
            Err(err) => eprintln!("Error opening file {}: {}", filename, err),
            Ok(file) => {
                let mut line_number = 1;
                for line in file.lines() {
                    let line: String = line?;
                    let should_number = arg.number || arg.number_nonblank && !line.is_empty();

                    let numbering = if should_number {
                        &format!("{:>6}\t", line_number)
                    } else {
                        ""
                    };

                    println!("{}{}", numbering, line);

                    if should_number {
                        line_number += 1;
                    }
                }
            }
        }
    }
    Ok(())
}
