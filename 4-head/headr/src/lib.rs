use clap::{Parser, value_parser};
use thiserror::Error;
use std::{error::Error, fs::File, io::{BufRead, BufReader}};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Error)]
pub enum BadFileError {
    #[error("{filename}: {from}")]
    Open {
        filename: String,
        #[source]
        from: Box<dyn Error>,
    },
}


#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// Name of files
    #[arg(default_values_t = ["-".to_string()])]
    files: Vec<String>,

    /// Number of lines to print
    #[arg(short='n', long, default_value_t = 10, conflicts_with = "bytes", value_parser=value_parser!(u32).range(1..))]
    lines : u32,

    /// Number of bytes to print
    #[arg(short = 'c', long, value_parser=value_parser!(u32).range(1..))]
    bytes: Option<u32>,
}

pub fn get_args() -> MyResult<Config> {
    Ok(Config::try_parse()?)
}

pub fn run(config: Config) -> MyResult<()> {
    let has_multiple_files = config.files.len() > 1;

    let results = config.files.iter().enumerate().map(|(i, filename)| -> MyResult<()> {
        if i > 0 {
            println!();
        }

        if has_multiple_files {
            println!("==> {} <==", filename);
        }

        let mut file = open(filename).map_err(|e| BadFileError::Open { filename: filename.clone(), from: e })?;
        if let Some(bytes) = config.bytes {
            let mut buf : Vec<u8> = vec![0; bytes as usize];
            let length = file.read(&mut buf)?;
            print!("{}", String::from_utf8_lossy(&buf[..length]));

        } else {
            LinesWithNewLine::from(file).take(config.lines as usize).try_for_each(|line| -> MyResult<()> {
                print!("{}", line?);
                Ok(())
            })?;
        }



        Ok(())
    });

    match results.collect::<MyResult<()>>() {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}


struct LinesWithNewLine (Box<dyn BufRead>);

impl From<Box<dyn BufRead>> for LinesWithNewLine {
    fn from(buf_read: Box<dyn BufRead>) -> Self {
        Self(buf_read)
    }
}

impl Iterator for LinesWithNewLine {
    type Item = MyResult<String>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = String::new();
        match self.0.read_line(&mut line) {
            Ok(0) => None,
            Ok(_) => Some(Ok(line)),
            Err(e) => Some(Err(e.into())),
        }
    }
}
