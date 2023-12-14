use std::collections::HashMap;

pub fn walk(contents: String, position: u32) -> u32 {
    let char_vec: Vec<char> = contents.chars().collect();
    let mut position_vec: Vec<u32> = vec![]; // i think i have to change this into a 2 dimensional vector containing the positions of 2 paths
                                             // then i iterate over this vector and save the new position her until they are equal,
                                             // and then increase the counter for each step and return the value of the counter
    position_vec.push(position, position);

    let mut width: i32 = 0;
    if let Some(first_line) = contents.lines().next() {
        width = first_line.len() as i32;
        println!("width is {}", width)
    }
    let mut counter = 0;
    let pt_hm = createpointerhm(width);
    let position1 = position.clone();
    let position2 = position.clone();
    //let println!("{:?}", pt_hm);
    //while { // the while loop has to go on until the position of the two paths are equal
    for (key, value) in pt_hm {
        let dx = (position as i32 + value) as usize;
        if let Some(next_char) = char_vec.get(dx) {
            if key.contains(*next_char) {
                println!("Hey I found a {} at position {}", next_char, dx);
                position_vec.push(position);
                counter += 1;
            }
            // if is_special_char(*next_char, "!@#$%^&-*()+=/") {
            //     //println!("special character {} found", *next_char);
            //     valid = true;
            // }
        };
    }
    //}

    counter
}

fn createpointerhm(width: i32) -> HashMap<String, i32> {
    if cfg!(target_os = "windows") {
        //vec![1, width + 1, -1, -width - 3]
        HashMap::from([
            ("-7J".to_string(), 1),
            ("|LJ".to_string(), width + 2),
            ("-LF".to_string(), -1),
            ("|7F".to_string(), -width - 4),
        ])
    } else {
        // Assume Linux or other Unix-like systems
        //vec![1, width + 1, -1, -width - 1]
        HashMap::from([
            ("-7J".to_string(), 1),
            ("|LJ".to_string(), width + 1),
            ("-LF".to_string(), -1),
            ("|7F".to_string(), -width - 3),
        ])
    }
}
