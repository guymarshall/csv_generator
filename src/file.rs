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


pub fn get_first_names() -> Result<Vec<String>, Error> {
    file_to_vector("first_names.txt")
}

pub fn get_middle_names() -> Result<Vec<String>, Error> {
    file_to_vector("middle_names.txt")
}

pub fn get_last_names() -> Result<Vec<String>, Error> {
    file_to_vector("last_names.txt")
}

/*
def file_to_list(filename: str):
    try:
        with open(filename) as file:
            words = [word.strip() for word in file]
    except FileNotFoundError:
        print(f"Error: The file '{filename}' does not exist.")
        return []
    return words

def get_names(filename: str):
    first_names = file_to_list(filename)
    if not first_names:
        raise Exception(f"Error: The file '{filename}' does not exist or is empty.")
    return first_names
*/