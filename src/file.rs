use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::process::exit;

fn file_to_vector(filename: &str) -> Result<Vec<String>, Error> {
    let file: File = File::open(filename)?;
    let reader: BufReader<File> = BufReader::new(file);

    let words: Vec<String> = reader
        .lines()
        .map(|line: Result<String, Error>| line.unwrap())
        .collect();

    Ok(words)
}

pub fn get_names(filename: &str) -> Vec<String> {
    file_to_vector(filename).unwrap_or_else(|error| {
        eprintln!("An error occurred when trying to open the file: {}", error);
        exit(1);
    })
}
