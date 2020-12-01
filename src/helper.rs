use std::env;
use std::fs;
use std::path::Path;

pub fn get_input_file(filename: &str) -> String {
    let src_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut input_file = src_dir.to_path_buf();
    input_file.push("inputs");
    input_file.push(filename);
    return fs::read_to_string(&input_file).expect("Input file missing.");
}

pub fn get_numbers_from_file(filename: &str) -> Vec<u32> {
    let contents = get_input_file(filename);
    return contents.lines().map(|line| line.parse().unwrap()).collect();
}

pub fn print_answer(name: &str, answer: u32) {
    println!("The answer for {} is: {}", name, answer);
}