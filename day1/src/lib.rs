use std::collections::HashMap;

pub fn add_numbers(string: String) -> i32 {
    let mut a: i32 = 0; // initialize var a
    let mut  ab: i32 = 0;
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
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let mut a: i32 = 0; // initialize var a
    let mut  ab: i32 = 0;
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