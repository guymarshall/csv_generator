use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn generate_initials(first_name: &String, middle_name: &String, last_name: &String) -> String {
    let mut result: String = String::new();

    if !first_name.is_empty() {
        result.push(first_name.chars().next().unwrap());
    }
    if !middle_name.is_empty() {
        result.push(middle_name.chars().next().unwrap());
    }
    if !last_name.is_empty() {
        result.push(last_name.chars().next().unwrap());
    }

    result
}

pub fn generate_csv(filename: &str, field_headings: &[&str], data: &[Vec<&str>]) {
    let path: &Path = Path::new(filename);
    let mut file: File = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let headings: String = field_headings.join(",");
    if let Err(why) = writeln!(file, "{}", headings) {
        panic!("couldn't write to {}: {}", path.display(), why);
    }

    for record in data {
        let line: String = record.join(",");
        if let Err(why) = writeln!(file, "{}", line) {
            panic!("couldn't write to {}: {}", path.display(), why);
        }
    }
}