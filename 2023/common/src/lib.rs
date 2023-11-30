use std::env;
use std::fs;

#[must_use]
pub fn challenge_input() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path = &args
        .get(1)
        .expect("⚠️Please pass a path to an input file! ⚠️");

    fs::read_to_string(file_path).expect("⚠️Unable to read input file! ⚠️")
}
