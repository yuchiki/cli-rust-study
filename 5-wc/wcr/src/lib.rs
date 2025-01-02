use std::error::Error;
use clap::Parser;

type MyResult<T> = Result<T, Box<dyn Error>>;


#[derive(Debug, Parser)]
#[command(author, version, about, long_about=None)]
pub struct Config {
    #[arg(default_values_t = ["-".to_string()])]
    files: Vec<String>,

    #[arg(short='c', long)]
    bytes: bool,

    #[arg(short='m', long, conflicts_with="bytes")]
    chars: bool,

    #[arg(short, long)]
    lines: bool,

    #[arg(short, long)]
    words: bool,
}

impl Config {
    pub fn parse_and_normalize () -> MyResult<Config> {
        let config = Config::try_parse()?;
        Ok(Config::normalize(config))
    }

    fn normalize (config: Self) -> Config {
        if [config.bytes, config.chars, config.lines, config.words].iter().all(|&x| !x) {
            Config { bytes: true, lines: true, words: true, ..config }
        } else {
            config
        }
    }
}


pub fn get_args() -> MyResult<Config> {
    Config::parse_and_normalize()
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);

    Ok(())
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

        fn files(self, files: Vec<String>) -> Self {
            ConfigBuilder { files, ..self }
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

    fn compare_configs(actual: Config, expected: Config) {
        assert_eq!(actual.files, expected.files);
        assert_eq!(actual.bytes, expected.bytes);
        assert_eq!(actual.chars, expected.chars);
        assert_eq!(actual.lines, expected.lines);
        assert_eq!(actual.words, expected.words);
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
        compare_configs(actual, expected.build());
    }
}
