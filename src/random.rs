use rand::prelude::*;
use rand::distributions::uniform::Uniform;

pub fn generate_random_number(min: i32, max: i32, return_string: bool) -> String {
    let mut rng = rand::thread_rng();
    let range = Uniform::from(min..=max);
    let result = range.sample(&mut rng);
    if return_string {
        result.to_string()
    } else {
        result.to_string()
    }
}