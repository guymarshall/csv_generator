use rand::prelude::*;
use std::ops::{Range, RangeInclusive};

use crate::file::file_to_vector;

pub fn random_number(min: i32, max: i32) -> i32 {
    rand::rng().random_range(min..max)
}

pub fn random_name(names: &[String]) -> String {
    let index: usize = rand::rng().random_range(0..names.len());
    names[index].to_string()
}

pub fn day_from_i32(day_int: i32) -> String {
    match day_int {
        0 => "Monday".to_string(),
        1 => "Tuesday".to_string(),
        2 => "Wednesday".to_string(),
        3 => "Thursday".to_string(),
        4 => "Friday".to_string(),
        5 => "Saturday".to_string(),
        _ => "Sunday".to_string(),
    }
}

pub fn random_room() -> String {
    let rooms: Vec<String> = file_to_vector("rooms.txt");

    let index: usize = rand::rng().random_range(0..rooms.len());
    rooms[index].to_owned()
}

pub fn random_length_random_vector() -> Vec<i32> {
    let mut random_number_generator: ThreadRng = rand::rng();
    let length_range: RangeInclusive<usize> = 1..=11;
    let length: usize = random_number_generator.random_range(length_range);

    (0..length)
        .map(|_| random_number_generator.random_range(1..=11))
        .collect()
}

pub fn random_teacher_type(type_type: &str) -> String {
    let names: [&str; 5] = [
        "Teacher",
        "Cover Teacher",
        "Trainee Teacher",
        "Head of Department",
        "Assistant Teacher",
    ];
    let display_names: [&str; 5] = ["Teacher", "Cover", "Trainee", "Head", "Assistant"];

    let mut random_number_generator: ThreadRng = rand::rng();
    let range: Range<usize> = 0..names.len();

    match type_type {
        "name" => (&names[random_number_generator.random_range(range)]).to_string(),
        "displayName" => (&display_names[random_number_generator.random_range(range)]).to_string(),
        _ => (&names[0]).to_string(),
    }
}

pub fn random_subject_name() -> String {
    // TODO: extract to subjects.txt
    let subjects: Vec<&str> = vec![
        "Maths",
        "Biology",
        "Chemistry",
        "Physics",
        "History",
        "Geography",
        "ICT",
        "German",
        "French",
        "Spanish",
        "DT",
        "PE",
        "English",
        "Personal Development",
        "RE",
    ];

    let index: usize = rand::rng().random_range(0..subjects.len());
    subjects[index].to_string()
}
