#![forbid(unsafe_code)]

use rand::distributions::uniform::Uniform;
use rand::prelude::*;
use std::ops::{Range, RangeInclusive};

pub fn random_number(min: i32, max: i32) -> i32 {
    let mut rng: ThreadRng = thread_rng();
    let range: Uniform<i32> = Uniform::from(min..=max);
    range.sample(&mut rng)
}

pub fn random_name(names: &[String]) -> String {
    let mut rng: ThreadRng = thread_rng();
    let range: Uniform<usize> = Uniform::new(0, names.len());
    let index: usize = rng.sample(range);
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

pub fn random_room() -> &'static str {
    let rooms: [&str; 49] = [
        "Ma1",
        "Ma2",
        "Ma3",
        "Ma4",
        "Ma5",
        "Ma6",
        "Ma7",
        "Ma8",
        "Ma9",
        "DT1",
        "DT2",
        "DT3",
        "DT4",
        "DT5",
        "IT1",
        "IT2",
        "IT3",
        "La1",
        "La2",
        "La3",
        "La4",
        "La5",
        "History1",
        "History2",
        "History3",
        "Geography1",
        "Geography2",
        "Geography3",
        "Sc1",
        "Sc2",
        "Sc3",
        "Sc4",
        "Sc5",
        "Sc6",
        "Sc7",
        "Sc8",
        "Eng1",
        "Eng2",
        "Eng3",
        "Eng4",
        "Eng5",
        "Eng6",
        "Eng7",
        "Eng8",
        "Music1",
        "Music2",
        "Drama1",
        "Drama2",
        "PE",
    ];

    let mut rng: ThreadRng = thread_rng();
    let range: RangeInclusive<usize> = 0..=rooms.len() - 1;
    rooms[rng.gen_range(range)]
}

pub fn random_length_random_vector() -> Vec<i32> {
    let mut random_number_generator: ThreadRng = thread_rng();
    let length_range: RangeInclusive<usize> = 1..=11;
    let length: usize = random_number_generator.gen_range(length_range);

    (0..length)
        .map(|_| random_number_generator.gen_range(1..=11))
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

    let mut random_number_generator: ThreadRng = thread_rng();
    let range: Range<usize> = 0..names.len();

    match type_type {
        "name" => (&names[random_number_generator.gen_range(range)]).to_string(),
        "displayName" => (&display_names[random_number_generator.gen_range(range)]).to_string(),
        _ => (&names[0]).to_string(),
    }
}

pub fn random_subject_name() -> String {
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
        "DT",
        "PE",
        "English",
        "Personal Development",
        "RE",
    ];
    let mut random_number_generator: ThreadRng = thread_rng();
    let index: usize = random_number_generator.gen_range(0..subjects.len());
    subjects[index].to_string()
}
