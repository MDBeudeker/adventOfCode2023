use std::fs;
use day2::count_cubes;
use std::collections::HashMap;

fn main() {

    let _cube_possibilities = HashMap::from([
        ("red".to_string(), 12),
        ("green".to_string(), 13),
        ("blue".to_string()  , 14),
    ]);

    let file_path  = "sourcefile.txt";
    let contents = fs::read_to_string(file_path).expect("File not found",);

    if let Some(first_line) = contents.lines().next() {
        println!("{}", first_line);
        let a =count_cubes("haha".to_string());
    } 

}
