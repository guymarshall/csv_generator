#![forbid(unsafe_code)]

use std::fs::File;
use std::io::{BufRead, BufReader};

fn file_to_vector(filename: &str) -> Vec<String> {
    let file: File = match File::open(filename) {
        Ok(file) => file,
        Err(error) => panic!("Couldn't open file: {}", error),
    };

    let reader: BufReader<File> = BufReader::new(file);
    let mut words: Vec<String> = Vec::new();

    for line in reader.lines() {
        let word: String = line.unwrap();
        words.push(word);
    }

    words
}

pub fn get_first_names() -> Vec<String> {
    file_to_vector("first_names.txt")
}

pub fn get_middle_names() -> Vec<String> {
    file_to_vector("middle_names.txt")
}

pub fn get_last_names() -> Vec<String> {
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