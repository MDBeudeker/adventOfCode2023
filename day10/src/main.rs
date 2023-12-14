use std::fs;

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("File not found");

    println!("{}", contents);
    let position = contents.chars().position(|c| c == 'S').unwrap();
    println!("position {}", position);

    if let Some(first_line) = contents.lines().next() {
        let width = first_line.len();
        println!("width is {}", width)
    }
}
