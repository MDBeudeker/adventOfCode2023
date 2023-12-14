use day10::walk;
use std::fs;

fn main() {
    let file_path = "input2.txt";
    let contents = fs::read_to_string(file_path).expect("File not found");

    println!("{}", contents);
    let position = contents.chars().position(|c| c == 'S').unwrap();
    println!("position {}", position);

    let max_distance = walk(contents, position as u32);

    println!("The maximum distance is {}", max_distance)
}
