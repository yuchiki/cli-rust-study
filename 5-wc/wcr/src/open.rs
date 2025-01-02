use std::{fs::File, io::{BufRead, BufReader}};

use thiserror::Error;
use std::error::Error;

use crate::MyResult;


#[derive(Debug, Error)]
pub enum FileError {
    #[error("{0}: {1}")]
    NotFound(String, #[source] Box<dyn Error>),
}


pub fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename).map_err(|e| FileError::NotFound(filename.to_string(), e.into()))?))),
    }
}
