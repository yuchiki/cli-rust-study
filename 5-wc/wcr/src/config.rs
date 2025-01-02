use clap::Parser;
use crate::MyResult;

#[derive(Debug, Parser, PartialEq)]
#[command(author, version, about, long_about=None)]
pub struct Config {
    #[arg(default_values_t = ["-".to_string()])]
    pub files: Vec<String>,

    #[arg(short='c', long)]
    pub bytes: bool,

    #[arg(short='m', long, conflicts_with="bytes")]
    pub chars: bool,

    #[arg(short, long)]
    pub lines: bool,

    #[arg(short, long)]
    pub words: bool,
}

impl Config {
    pub fn parse_and_normalize () -> MyResult<Config> {
        let config = Config::try_parse()?;
        Ok(Config::normalize(config))
    }

    pub fn normalize (config: Self) -> Config {
        if [config.bytes, config.chars, config.lines, config.words].iter().all(|&x| !x) {
            Config { bytes: true, lines: true, words: true, ..config }
        } else {
            config
        }
    }
}
