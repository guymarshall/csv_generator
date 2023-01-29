use std::ops::{Range, RangeInclusive};
use rand::prelude::*;
use rand::distributions::uniform::Uniform;

pub fn generate_random_number(min: i32, max: i32, return_string: bool) -> String {
    let mut rng: ThreadRng = thread_rng();
    let range: Uniform<i32> = Uniform::from(min..=max);
    let result: i32 = range.sample(&mut rng);
    if return_string {
        result.to_string()
    } else {
        result.to_string()
    }
}

fn get_random_name(names: &Vec<&str>) -> String {
    let mut rng: ThreadRng = thread_rng();
    let range: Uniform<usize> = Uniform::new(0, names.len());
    let index: usize = rng.sample(range);
    names[index].to_string()
}

fn random_day(include_weekends: bool) -> String {
    let days_of_week: [&str; 5] = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];
    let mut rng: ThreadRng = thread_rng();
    let range: Uniform<usize> = if include_weekends {
        Uniform::new(0, 7)
    } else {
        Uniform::new(0, 5)
    };
    let index: usize = rng.sample(range);
    days_of_week[index].to_string()
}

fn random_room() -> &'static str {
    let rooms: [&str; 49] = ["Ma1", "Ma2", "Ma3", "Ma4", "Ma5", "Ma6", "Ma7", "Ma8", "Ma9", "DT1", "DT2", "DT3", "DT4", "DT5", "IT1", "IT2", "IT3", "La1", "La2", "La3", "La4", "La5", "History1", "History2", "History3", "Geography1", "Geography2", "Geography3", "Sc1", "Sc2", "Sc3", "Sc4", "Sc5", "Sc6", "Sc7", "Sc8", "Eng1", "Eng2", "Eng3", "Eng4", "Eng5", "Eng6", "Eng7", "Eng8", "Music1", "Music2", "Drama1", "Drama2", "PE"];

    let mut rng: ThreadRng = thread_rng();
    let range: RangeInclusive<usize> = 0..=rooms.len() - 1;
    &rooms[rng.gen_range(range)]
}

fn generate_random_length_random_vector() -> Vec<i32> {
    let mut rng: ThreadRng = thread_rng();
    let length_range: RangeInclusive<usize> = 1..=11;
    let length: usize = rng.gen_range(length_range);

    let mut output: Vec<i32> = vec![];
    for _ in 0..length {
        let value_range: RangeInclusive<i32> = 1..=11;
        output.push(rng.gen_range(value_range));
    }

    output
}

fn random_teacher_type(type_: &str) -> &str {
    let names: [&str; 4] = ["Teacher", "Cover Teacher", "Trainee Teacher", "Head of Department"];
    let display_names: [&str; 4] = ["Teacher", "Cover", "Trainee", "Head"];

    let mut rng: ThreadRng = thread_rng();
    let range: Range<usize> = 0..names.len();

    match type_ {
        "name" => &names[rng.gen_range(range)],
        "displayName" => &display_names[rng.gen_range(range)],
        _ => &names[0],
    }
}

fn generate_initials(first_name: &str, middle_name: &str, last_name: &str) -> String {
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