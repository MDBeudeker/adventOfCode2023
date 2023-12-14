use day09::interpolate;
use std::fs;

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("File not found");

    let mut sum1 = 0;
    let mut sum2 = 0;
    for line in contents.lines() {
        let (a, b) = interpolate(line.to_string());
        sum1 = sum1 + a;
        sum2 = sum2 + b;
    }

    println!("sum one is {}", sum1);
    println!("sum two is {}", sum2);
}
