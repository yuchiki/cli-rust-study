use core::fmt;
use std::{error::Error, fs::File, io::{BufRead, BufReader}};
use thiserror::Error;

mod config;
use config::Config;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Error)]
pub enum FileError {
    #[error("{0}: {1}")]
    NotFound(String, #[source] Box<dyn Error>),
}

#[derive(Debug)]
struct FileInfo {
    name: String,
    bytes: u64,
    chars: u64,
    lines: u64,
    words: u64,
}

struct FilePrinter<'a> {
    config: &'a Config,
    file: &'a FileInfo,
}

impl fmt::Display for FilePrinter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let format_param = |n, should_print | if should_print { format!("{:8}", n) } else { "".to_string() };
        let format_name =  |name: String| if name == *"-" {"".to_string()} else {" ".to_string() +  &name};
        write!(f, "{}{}{}{}{}",
        format_param(self.file.lines, self.config.lines),
        format_param(self.file.words, self.config.words),
        format_param(self.file.bytes, self.config.bytes),
        format_param(self.file.chars, self.config.chars),
        format_name(self.file.name.clone()))
    }
}


pub fn get_args() -> MyResult<Config> {
    Config::parse_and_normalize()
}

pub fn run(config: Config) -> MyResult<()> {
    let file_infos : Vec<MyResult<FileInfo>> = config.files.iter()
        .map(|filename| -> MyResult<(&str, Box<dyn BufRead>)> { Ok((filename, open(filename).map_err(|e| FileError::NotFound(filename.to_string(), e))?)) })
        .map(|file| {
        let (filename, mut file) = file?;

        let mut bytes = 0;
        let mut chars = 0;
        let mut lines = 0;
        let mut words = 0;

        let mut buf = String::new();
        while file.read_line(&mut buf)? > 0 {
            bytes += buf.len() as u64;
            chars += buf.chars().count() as u64;
            lines += 1;
            words += buf.split_whitespace().count() as u64;
            buf.clear();
        }

        Ok(FileInfo {
            name: filename.to_string(),
            bytes,
            chars,
            lines,
            words,
        })
    }).collect();

    file_infos.iter().for_each(|info| {
        match info {
            Err(e) => {
                eprintln!("{}", e);
            },
            Ok(info) => {
                let printer =  FilePrinter { config: &config, file: info};
                println!("{}", printer);
            }
        }
    });

    if file_infos.len() > 1 {
        let total = file_infos.iter().fold(FileInfo { name: "total".to_string(), bytes: 0, chars: 0, lines: 0, words: 0 }, |acc, info| {
            match info {
                Err(_) => acc,
                Ok(info) => FileInfo {
                    name: "total".to_string(),
                    bytes: acc.bytes + info.bytes,
                    chars: acc.chars + info.chars,
                    lines: acc.lines + info.lines,
                    words: acc.words + info.words,
                }
            }
        });

        let printer =  FilePrinter { config: &config, file: &total};
        println!("{}", printer);
    }

    Ok(())
}


fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    struct ConfigBuilder {
        files: Vec<String>,
        bytes: bool,
        chars: bool,
        lines: bool,
        words: bool,
    }

    impl ConfigBuilder {
        fn new() -> Self {
            ConfigBuilder {
                files: vec!["-".to_string()],
                bytes: false,
                chars: false,
                lines: false,
                words: false,
            }
        }

        fn bytes(self) -> Self {
            ConfigBuilder { bytes: true, ..self }
        }

        fn chars(self) -> Self {
            ConfigBuilder { chars: true, ..self }
        }

        fn lines(self) -> Self {
            ConfigBuilder { lines: true, ..self }
        }

        fn words(self) -> Self {
            ConfigBuilder { words: true, ..self }
        }

        fn build(self) -> Config {
            Config {
                files: self.files,
                bytes: self.bytes,
                chars: self.chars,
                lines: self.lines,
                words: self.words,
            }
        }
    }

    fn base() -> ConfigBuilder {
        ConfigBuilder::new()
    }

    #[rstest]
    #[case(base(), base().bytes().lines().words())]
    #[case(base().bytes(), base().bytes())]
    #[case(base().chars(), base().chars())]
    #[case(base().lines(), base().lines())]
    #[case(base().words(), base().words())]
    #[case(base().bytes().lines(), base().bytes().lines())]
    fn test_normalize(#[case] input: ConfigBuilder, #[case] expected: ConfigBuilder) {
        let actual = Config::normalize(input.build());
        assert_eq!(actual, expected.build());
    }
}
