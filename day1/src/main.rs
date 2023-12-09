use std::fs;
use day1::add_numbers;

fn main() {

    let file_path  = "sourcefile.txt";
    let contents = fs::read_to_string(file_path).expect("File not found",);

    println!("{}", add_numbers(contents));
}

