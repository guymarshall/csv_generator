use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn generate_initials(first_name: String, middle_name: String, last_name: String) -> String {
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

pub fn add_quotes(input: &str) -> String {
    format!("\"{}\"", input)
}

pub fn vector_to_unique_string_with_quotes<T: ToString + Eq + std::hash::Hash>(
    input: &[T],
) -> String {
    let mut unique_elements: std::collections::HashSet<&T> = std::collections::HashSet::new();
    let output: String = input
        .iter()
        .filter(|&value| unique_elements.insert(value))
        .map(|value| value.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    format!("\"{}\"", output.trim_end())
}

pub fn generate_curriculum_csv(filename: &str, field_headings: Vec<&str>, data: Vec<Vec<i32>>) {
    let path: &Path = Path::new(filename);
    let mut file: File = match File::create(path) {
        Err(why) => panic!("couldn't create {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let headings_including_trailing_comma: String = field_headings
        .iter()
        .map(|&heading| heading.to_string() + ",")
        .collect();
    let headings: String = headings_including_trailing_comma[0..].to_string();
    if let Err(why) = writeln!(file, "{}", headings) {
        panic!("couldn't write to {}: {}", path.display(), why);
    }

    for record in data {
        let line_including_trailing_comma: String =
            record.iter().map(|&cell| cell.to_string() + ",").collect();
        let line: String = line_including_trailing_comma[0..].to_string();
        if let Err(why) = writeln!(file, "{}", line) {
            panic!("couldn't write to {}: {}", path.display(), why);
        }
    }
}

pub fn generate_period_schedule_csv(
    filename: &str,
    field_headings: Vec<&str>,
    data: Vec<Vec<(i32, String, i32)>>,
) {
    let path: &Path = Path::new(filename);
    let mut file: File = match File::create(path) {
        Err(why) => panic!("couldn't create {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let headings_including_trailing_comma: String = field_headings
        .iter()
        .map(|&heading| heading.to_string() + ",")
        .collect();
    let headings: String = headings_including_trailing_comma[0..].to_string();
    if let Err(why) = writeln!(file, "{}", headings) {
        panic!("couldn't write to {}: {}", path.display(), why);
    }

    for record in data {
        let line_including_trailing_comma: String = record
            .iter()
            .map(|cell| format!("{}, {}, {}", cell.0, cell.1, cell.2))
            .collect::<Vec<String>>()
            .join(",");
        let line: String = line_including_trailing_comma[0..].to_string();
        if let Err(why) = writeln!(file, "{}", line) {
            panic!("couldn't write to {}: {}", path.display(), why);
        }
    }
}

pub fn generate_room_csv(
    filename: &str,
    field_headings: Vec<&str>,
    data: Vec<Vec<(i32, String, i32)>>,
) {
    let path: &Path = Path::new(filename);
    let mut file: File = match File::create(path) {
        Err(why) => panic!("couldn't create {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let headings_including_trailing_comma: String = field_headings
        .iter()
        .map(|&heading| heading.to_string() + ",")
        .collect();
    let headings: String = headings_including_trailing_comma[0..].to_string();
    if let Err(why) = writeln!(file, "{}", headings) {
        panic!("couldn't write to {}: {}", path.display(), why);
    }

    for record in data {
        let line_including_trailing_comma: String = record
            .iter()
            .map(|cell| format!("{}, {}, {}", cell.0, cell.1, cell.2))
            .collect::<Vec<String>>()
            .join(",");
        let line: String = line_including_trailing_comma[0..].to_string();
        if let Err(why) = writeln!(file, "{}", line) {
            panic!("couldn't write to {}: {}", path.display(), why);
        }
    }
}

pub fn generate_student_csv(
    filename: &str,
    field_headings: Vec<&str>,
    data: Vec<Vec<(String, String, String, String, String)>>,
) {
    let path: &Path = Path::new(filename);
    let mut file: File = match File::create(path) {
        Err(why) => panic!("couldn't create {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let headings_including_trailing_comma: String = field_headings
        .iter()
        .map(|&heading| heading.to_string() + ",")
        .collect();
    let headings: String = headings_including_trailing_comma[0..].to_string();
    if let Err(why) = writeln!(file, "{}", headings) {
        panic!("couldn't write to {}: {}", path.display(), why);
    }

    for record in data {
        let line_including_trailing_comma: String = record
            .iter()
            .map(|cell| format!("{}, {}, {}, {}, {}", cell.0, cell.1, cell.2, cell.3, cell.4))
            .collect::<Vec<String>>()
            .join(",");
        let line: String = line_including_trailing_comma[0..].to_string();
        if let Err(why) = writeln!(file, "{}", line) {
            panic!("couldn't write to {}: {}", path.display(), why);
        }
    }
}

pub fn generate_subject_csv(
    filename: &str,
    field_headings: Vec<&str>,
    data: Vec<Vec<(i32, String, i32, String, i32, String)>>,
) {
    let path: &Path = Path::new(filename);
    let mut file: File = match File::create(path) {
        Err(why) => panic!("couldn't create {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let headings_including_trailing_comma: String = field_headings
        .iter()
        .map(|&heading| heading.to_string() + ",")
        .collect();
    let headings: String = headings_including_trailing_comma[0..].to_string();
    if let Err(why) = writeln!(file, "{}", headings) {
        panic!("couldn't write to {}: {}", path.display(), why);
    }

    for record in data {
        let line_including_trailing_comma: String = record
            .iter()
            .map(|cell| {
                format!(
                    "{}, {}, {}, {}, {}, {}",
                    cell.0, cell.1, cell.2, cell.3, cell.4, cell.5
                )
            })
            .collect::<Vec<String>>()
            .join(",");
        let line: String = line_including_trailing_comma[0..].to_string();
        if let Err(why) = writeln!(file, "{}", line) {
            panic!("couldn't write to {}: {}", path.display(), why);
        }
    }
}

pub fn generate_teacher_csv(
    filename: &str,
    field_headings: Vec<&str>,
    data: Vec<Vec<(i32, String, String, String, String, i32, String, String)>>,
) {
    let path: &Path = Path::new(filename);
    let mut file: File = match File::create(path) {
        Err(why) => panic!("couldn't create {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let headings_including_trailing_comma: String = field_headings
        .iter()
        .map(|&heading| heading.to_string() + ",")
        .collect();
    let headings: String = headings_including_trailing_comma[0..].to_string();
    if let Err(why) = writeln!(file, "{}", headings) {
        panic!("couldn't write to {}: {}", path.display(), why);
    }

    for record in data {
        let line_including_trailing_comma: String = record
            .iter()
            .map(|cell| {
                format!(
                    "{}, {}, {}, {}, {}, {}, {}, {}",
                    cell.0, cell.1, cell.2, cell.3, cell.4, cell.5, cell.6, cell.7
                )
            })
            .collect::<Vec<String>>()
            .join(",");
        let line: String = line_including_trailing_comma[0..].to_string();
        if let Err(why) = writeln!(file, "{}", line) {
            panic!("couldn't write to {}: {}", path.display(), why);
        }
    }
}
pub fn generate_teacher_type_csv(
    filename: &str,
    field_headings: Vec<&str>,
    data: Vec<Vec<(i32, String, String)>>,
) {
    let path: &Path = Path::new(filename);
    let mut file: File = match File::create(path) {
        Err(why) => panic!("couldn't create {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let headings_including_trailing_comma: String = field_headings
        .iter()
        .map(|&heading| heading.to_string() + ",")
        .collect();
    let headings: String = headings_including_trailing_comma[0..].to_string();
    if let Err(why) = writeln!(file, "{}", headings) {
        panic!("couldn't write to {}: {}", path.display(), why);
    }

    for record in data {
        let line_including_trailing_comma: String = record
            .iter()
            .map(|cell| format!("{}, {}, {}", cell.0, cell.1, cell.2))
            .collect::<Vec<String>>()
            .join(",");
        let line: String = line_including_trailing_comma[0..].to_string();
        if let Err(why) = writeln!(file, "{}", line) {
            panic!("couldn't write to {}: {}", path.display(), why);
        }
    }
}
