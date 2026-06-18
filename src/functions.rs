use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::{PeriodSchedule, Room, Student, Subject, Teacher, TeacherType};

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

pub fn add_quotes(input: String) -> String {
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

pub fn generate_period_schedule_csv(
    filename: &str,
    field_headings: Vec<&str>,
    data: Vec<PeriodSchedule>,
) {
    let path: &Path = Path::new(filename);
    let mut file: File = match File::create(path) {
        Err(why) => panic!("couldn't create {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let headings_including_trailing_comma: String = field_headings
        .iter()
        .map(|heading: &&str| heading.to_string() + ",")
        .collect();
    let headings: String = headings_including_trailing_comma[0..].to_string();
    if let Err(why) = writeln!(file, "{}", headings) {
        panic!("couldn't write to {}: {}", path.display(), why);
    }

    data.into_iter()
        .for_each(|period_schedule: PeriodSchedule| {
            let line: String = format!(
                "{}, {}, {}",
                period_schedule.id, period_schedule.day_of_week, period_schedule.number_of_periods
            );
            if let Err(why) = writeln!(file, "{}", line) {
                panic!("couldn't write to {}: {}", path.display(), why);
            }
        });
}

pub fn generate_room_csv(filename: &str, field_headings: Vec<&str>, data: Vec<Room>) {
    let path: &Path = Path::new(filename);
    let mut file: File = match File::create(path) {
        Err(why) => panic!("couldn't create {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let headings_including_trailing_comma: String = field_headings
        .iter()
        .map(|heading: &&str| heading.to_string() + ",")
        .collect();
    let headings: String = headings_including_trailing_comma[0..].to_string();
    if let Err(why) = writeln!(file, "{}", headings) {
        panic!("couldn't write to {}: {}", path.display(), why);
    }

    data.into_iter().for_each(|room: Room| {
        let line: String = format!("{}, {}, {}", room.id, room.name, room.maximum_class_size);
        if let Err(why) = writeln!(file, "{}", line) {
            panic!("couldn't write to {}: {}", path.display(), why);
        }
    });
}

pub fn generate_student_csv(filename: &str, field_headings: Vec<&str>, data: Vec<Student>) {
    let path: &Path = Path::new(filename);
    let mut file: File = match File::create(path) {
        Err(why) => panic!("couldn't create {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let headings_including_trailing_comma: String = field_headings
        .iter()
        .map(|heading: &&str| heading.to_string() + ",")
        .collect();
    let headings: String = headings_including_trailing_comma[0..].to_string();
    if let Err(why) = writeln!(file, "{}", headings) {
        panic!("couldn't write to {}: {}", path.display(), why);
    }

    data.into_iter().for_each(|student: Student| {
        let line: String = format!(
            "{}, {}, {}, {}, {}",
            student.id, student.first_name, student.middle_names, student.surname, student.initials
        );
        if let Err(why) = writeln!(file, "{}", line) {
            panic!("couldn't write to {}: {}", path.display(), why);
        }
    });
}

pub fn generate_subject_csv(filename: &str, field_headings: Vec<&str>, data: Vec<Subject>) {
    let path: &Path = Path::new(filename);
    let mut file: File = match File::create(path) {
        Err(why) => panic!("couldn't create {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let headings_including_trailing_comma: String = field_headings
        .iter()
        .map(|heading: &&str| heading.to_string() + ",")
        .collect();
    let headings: String = headings_including_trailing_comma[0..].to_string();
    if let Err(why) = writeln!(file, "{}", headings) {
        panic!("couldn't write to {}: {}", path.display(), why);
    }

    data.into_iter().for_each(|subject: Subject| {
        let line: String = format!(
            "{}, {}, {}, {}, {}, {}",
            subject.id,
            subject.subject_name,
            subject.subject_year,
            subject.set,
            subject.maximum_class_size,
            subject.rooms_taught
        );
        if let Err(why) = writeln!(file, "{}", line) {
            panic!("couldn't write to {}: {}", path.display(), why);
        }
    });
}

pub fn generate_teacher_csv(filename: &str, field_headings: Vec<&str>, data: Vec<Teacher>) {
    let path: &Path = Path::new(filename);
    let mut file: File = match File::create(path) {
        Err(why) => panic!("couldn't create {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let headings_including_trailing_comma: String = field_headings
        .iter()
        .map(|heading: &&str| heading.to_string() + ",")
        .collect();
    let headings: String = headings_including_trailing_comma[0..].to_string();
    if let Err(why) = writeln!(file, "{}", headings) {
        panic!("couldn't write to {}: {}", path.display(), why);
    }

    data.into_iter().for_each(|teacher: Teacher| {
        let line: String = format!(
            "{}, {}, {}, {}, {}, {}, {}, {}",
            teacher.id,
            teacher.first_name,
            teacher.middle_name,
            teacher.surname,
            teacher.initials,
            teacher.teacher_type_id,
            teacher.subject_taught_ids,
            teacher.room_taught_ids
        );
        if let Err(why) = writeln!(file, "{}", line) {
            panic!("couldn't write to {}: {}", path.display(), why);
        }
    });
}

pub fn generate_teacher_type_csv(
    filename: &str,
    field_headings: Vec<&str>,
    data: Vec<TeacherType>,
) {
    let path: &Path = Path::new(filename);
    let mut file: File = match File::create(path) {
        Err(why) => panic!("couldn't create {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let headings_including_trailing_comma: String = field_headings
        .iter()
        .map(|heading: &&str| heading.to_string() + ",")
        .collect();
    let headings: String = headings_including_trailing_comma[0..].to_string();
    if let Err(why) = writeln!(file, "{}", headings) {
        panic!("couldn't write to {}: {}", path.display(), why);
    }

    data.into_iter().for_each(|teacher_type: TeacherType| {
        let line: String = format!(
            "{}, {}, {}",
            teacher_type.id, teacher_type.name, teacher_type.display_name
        );
        if let Err(why) = writeln!(file, "{}", line) {
            panic!("couldn't write to {}: {}", path.display(), why);
        }
    });
}
