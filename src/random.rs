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
