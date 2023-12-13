use std::fs;
use day09::{interpolate};

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("File not found");

    let sum_history = interpolate(contents.clone());

    println!("sum is {}", sum_history);
}
