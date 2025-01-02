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
