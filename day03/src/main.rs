use std::fs;
use day03::{sum_parts,sum_gear_parts};

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("File not found");

    let sum: u32 = sum_parts(contents.clone());
    println!("the sum of all parts is {}", sum);
    let sum: u32 = sum_gear_parts(contents);
    println!("the sum of all parts with a gear is {}", sum);
    
    
}
