#![forbid(unsafe_code)]

use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn file_to_vector(filename: &str) -> Result<Vec<String>, Error> {
    let file: File = File::open(filename)?;
    let reader: BufReader<File> = BufReader::new(file);
    let mut words: Vec<String> = Vec::new();

    for line in reader.lines() {
        let word: String = line?;
        words.push(word);
    }

    Ok(words)
}

pub fn get_names(filename: &str) -> Result<Vec<String>, Error> {
    file_to_vector(filename)
}