#![forbid(unsafe_code)]

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

/*
def generate_initials(first_name: str, middle_name: str, last_name: str) -> str:
    return f"{first_name[0]}{middle_name[0]}{last_name[0]}"

def generate_csv(filename: str, field_headings: list[str], data: list[object]) -> None:
    try:
        with open(filename, "w") as file:
            headings = ",".join(field_headings)
            file.write(headings + "\n")

            for record in data:
                if type(record) == str:
                    record = add_quotes(record)
                line = ",".join(record)
                file.write(line + "\n")
    except FileNotFoundError as e:
        print(f"File not found: {e}")
    except PermissionError as e:
        print(f"Permission error: {e}")
    except Exception as e:
        print(f"Unexpected error: {e}")

def add_quotes(s: str) -> str:
    return f'"{s}"'

def get_count(prompt: str) -> int:
    count = int(input(prompt))
    if count <= 0:
        print("Count must be greater than 0. Quitting program.")
        quit()
    return count

*/