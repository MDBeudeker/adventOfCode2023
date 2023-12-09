use std::fs;
use day1::add_numbers;
use day1::add_numbers_2;

fn main() {

    let file_path  = "sourcefile.txt";
    let contents = fs::read_to_string(file_path).expect("File not found",);

    println!("Sum of all digits: {}", add_numbers(contents.clone()));
    println!("Sum of all numbers also as strings: {}", add_numbers_2(contents.clone()));
}

