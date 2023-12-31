use std::fs;
use day2::{count_cubes,sum_cubes};

fn main() {

    let file_path  = "sourcefile.txt";
    let contents = fs::read_to_string(file_path).expect("File not found",);

    let mut sum: i32 = 0;
    // if let Some(first_line) = contents.lines().next() {
    //     //println!("{}", first_line);
    //     sum = sum + count_cubes(first_line.to_string());
    // } 

    for line in contents.lines(){
        sum = sum + count_cubes(line.to_string());
    } 

    for line in contents.lines(){
        sum = sum + sum_cubes(line.to_string());
    } 
    println!("the sum of all valid game numbers is {}", sum)
}
