pub fn sum_parts(contents: String) -> u32 {
    let mut width: i32 = 0;

    if let Some(first_line) = contents.lines().next() {
        width = first_line.len() as i32;
    }
    println!("{}", width);

    let mut parts: Vec<u32> = Vec::new();
    let char_vec: Vec<char> = contents.chars().collect();
    let mut position = 0;

    let pointer_vec = createpointervec(width);

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
                    if is_special_char(*next_char, "!@#$%^&-*()+=/") {
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
    let sum: u32 = parts.iter().sum();
    sum
}

pub fn sum_gear_parts(contents: String) -> u32 {
    let mut width: i32 = 0;

    if let Some(first_line) = contents.lines().next() {
        width = first_line.len() as i32;
    }
    println!("{}", width);

    let mut parts: Vec<(u32, u32)> = Vec::new();
    let char_vec: Vec<char> = contents.chars().collect();
    let mut position = 0;

    let pointer_vec = createpointervec(width);

    let mut empty = true;
    let mut valid = false;
    let mut anothernumber = false;
    let mut currentpart = 0;
    let mut gearposition: i32 = 0;

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
                    if is_special_char(*next_char, "*") {
                        //println!("special character {} found", *next_char);
                        valid = true;
                        gearposition = position as i32 + pointer;
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
                };
            }
            if valid && !anothernumber {
                parts.push((currentpart, gearposition as u32));
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
    //println!("{:?}", parts.clone());
    println!("removing duplicate parts...");

    let mut duplicates = Vec::new();
    let mut singles = Vec::new();

    for &(_, second_element) in &parts {
        if singles.contains(&second_element) {
            // If the value is already in singles, move it to duplicates
            if !duplicates.contains(&second_element) {
                duplicates.push(second_element);
            }
        } else {
            // If the value is not in singles, add it
            singles.push(second_element);
        }
    }

    let mut filtered_vector: Vec<_> = parts
        .clone()
        .into_iter()
        .filter(|&(_, second_element)| duplicates.contains(&second_element))
        .collect();
    filtered_vector.sort_by(|a, b| a.1.cmp(&b.1));
    //println!("Original vector: {:?}", parts);
    //println!("Filtered vector: {:?}", filtered_vector.clone());

    let mut sum: u32 = 0;

    let mut counter = 0;
    for window in filtered_vector.windows(2) {
        if counter % 2 == 0 {
            sum += window[0].0 * window[1].0;
            //println!("sum {} is{} times {}", sum, window[0].0, window[1].0)
        }
        counter += 1
    }

    //147787784 too high
    sum
}

fn is_special_char(c: char, query: &str) -> bool {
    query.contains(c)
}

fn createpointervec(width: i32) -> Vec<i32> {
    if cfg!(target_os = "windows") {
        println!("hoi");
        vec![
            0,
            width + 2,
            width + 1,
            width,
            -2,
            -width - 4,
            -width - 3,
            -width - 2,
        ]
    } else {
        // Assume Linux or other Unix-like systems
        println!("doei");
        vec![
            0,
            width + 1,
            width + 0,
            width - 1,
            -2,
            -width - 3,
            -width - 2,
            -width - 1,
        ]
    }
}
