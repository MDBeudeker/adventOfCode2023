use std::fs;

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("File not found");
    let mut width: i32 = 0;

    if let Some(first_line) = contents.lines().next() {
        width = first_line.len() as i32;
    }
    println!("{}", width);

    let mut parts: Vec<u32> = Vec::new();
    let mut partscounter: usize = 0;
    let char_vec: Vec<char> = contents.chars().collect();
    let mut position = 0;

    let pointer_vec = vec![
        0,
        width + 2, // extra windows offset
        width + 1,
        width,
        // width -1, // linux offset
        -2,
        -width - 4, // extra windows offset
        -width - 3,
        -width - 2,
        //-width - 1, // linux offset
    ];

    let mut empty = true;
    let mut valid = false;
    let mut anothernumber = false;
    let mut currentpart = 0;

    while position < char_vec.len() {
        if let Some(digit) = char_vec[position].to_digit(10) {
            if empty {
                currentpart = digit;
                empty = false;
            }

            position += 1;
            //println!("current digit {} ,position {}", digit, position);

            let mut pointercounter = 0;
            for &pointer in &pointer_vec {
                let next_index = (position as i32 + pointer) as usize;
                if let Some(next_char) = char_vec.get(next_index) {
                    //println!("current digit {}, next to {}", digit, next_char);
                    if is_special_char(*next_char) {
                        //println!("special character {} found", *next_char);
                        valid = true;
                    }
                    if pointercounter == 0 {
                        if let Some(part) = next_char.to_digit(10) {
                            currentpart = currentpart * 10 + part;
                            //println!("current part {}", currentpart);
                            anothernumber = true;
                        } else {
                            anothernumber = false
                        }
                    }
                    pointercounter += 1;
                    //println!("{:?}", parts);
                };
            }
            if valid && !anothernumber {
                parts.push(currentpart);
                valid = false;
                empty = true;
            } else if !valid && !anothernumber {
                currentpart = 0;
                empty = true;
            }
        } else {
            position += 1;
        }
    }
    println!("{:?}", parts);
    let sum: u32 = parts.iter().sum();

    println!("sum: {}", sum);
}

fn is_special_char(c: char) -> bool {
    let special_chars = "!@#$%^&-*()+=/";
    special_chars.contains(c)
}
