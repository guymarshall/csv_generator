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

pub fn get_names(filename: &str) -> Vec<String> {
    file_to_vector(filename).unwrap_or_else(|error| {
        eprintln!("An error occurred when trying to open the file: {}", error);
        std::process::exit(1);
    })
}