use std::collections::HashMap;

pub fn add_numbers(string: String) -> i32 {
    let mut a: i32 = 0; // initialize var a
    let mut ab: i32 = 0;
    let mut sum:i32 = 0;
    let mut trigger: bool = false;
    
    for c  in string.chars() {
        if let Some(digit) =  c.to_digit(10) {
            if !trigger {
                a = digit as i32;  // if char is the 1st number in the string, save it to 'a'
                trigger = true;             // 1st character has been found, don't save a on the next char
            }
            let b: i32 = digit as i32;
            ab = a *10 + b;
        }
        //println!("{c}")
        if (c as i32) == 10 {
            // println!("{ab}");
            sum = sum + ab;
            trigger = false // whenever a newline char occurs, set the trigger to false
        }
    }
    
    return sum
}

pub fn add_numbers_2(string: String) -> i32 {
    let numbers_dict = HashMap::from([
        ("zero".to_string() , 0),
        ("one".to_string()  , 1),
        ("two".to_string()  , 2),
        ("three".to_string(), 3),
        ("four".to_string() , 4),
        ("five".to_string() , 5),
        ("six".to_string()  , 6),
        ("seven".to_string(), 7),
        ("eight".to_string(), 8),
        ("nine".to_string() , 9),
    ]);
    
    let mut sum:i32 = 0;
    
    for line  in string.lines() {
    
        let mut a = 0;
        let mut b = 0;
        
        let mut matches: Vec<(i32, usize)> = Vec::new();
        for (word, number) in &numbers_dict {

            let mut position = 0;
            while let Some(found_position) = line[position..].find(word) {
                position += found_position;
                matches.push((*number, position));
                position += word.len();
            }

            let number_str = number.to_string();
            position = 0;
            while let Some (found_position) = line[position..].find(&number_str) {
                position += found_position;
                matches.push((*number, position));
                position += number_str.len();
            }
        }
        matches.sort_by(|a, b| a.1.cmp(&b.1));
        if let Some((first,_)) = matches.first() {
            a = *first;
        }
        if let Some((last, _)) = matches.last() {
            b = *last;
        }
        
        let ab = a *10 + b;
        sum = sum + ab;
    }
    
    return sum
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let contents: String = "eight9fhstbssrplmdlncmmqqnklb39ninejz".to_string();

        assert_eq!(18, add_numbers(contents));
    }
    
    #[test]
    fn error() {
        let contents: String = "eight9fhstbssrplmdlncmmqqnklb39ninejz".to_string();

        assert_eq!(18, add_numbers(contents));
    }
}