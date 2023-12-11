use std::collections::HashMap;

pub fn count_cubes (string: String) -> i32{
    let gamecontents: Vec<&str> = string.split(": ").collect();
    let turns: Vec<&str> =  gamecontents[1].split("; ").collect();
    let mut counter = 0;
    let gameid: i32 = gamecontents[0].split_whitespace().nth(1).unwrap().parse().unwrap(); // chatgpt magic

    let mut valid: bool = true;

    println!("game id is {:?}", gameid);
    for turn in turns {
        counter = counter+1;
        valid = verify_turn(turn);
        println!("turn {}: {}, valid: {}", counter, turn, valid);
        if !valid {break}
    }
    //println!("{}", &gamecontents.next().unwrap());
    if valid {
        return gameid
    } else {return 0}
}

fn verify_turn(turn:&str) -> bool {

    let cube_possibilities = HashMap::from([
        ("red".to_string(), 12),
        ("green".to_string(), 13),
        ("blue".to_string()  , 14),
    ]);

    let sets: Vec<&str> = turn.split(", ").collect();

    for set in sets{
        //println!("{}", set);
        for (color, &count) in &cube_possibilities {
            let elements: Vec<&str> = set.split_whitespace().collect();
            if elements[1] == color {
                if elements[0].parse::<i32>().unwrap() > count {
                    return false
                }
            }
        }
    }

    true

}

pub fn sum_cubes (string: String) -> i32{
    let gamecontents: Vec<&str> = string.split(": ").collect();
    let turns: Vec<&str> =  gamecontents[1].split("; ").collect();
    let mut counter = 0;
    let gameid: i32 = gamecontents[0].split_whitespace().nth(1).unwrap().parse().unwrap(); // chatgpt magic
    
    let mut sumcubes: i32 = 0;
    
    println!("game id is {:?}", gameid);
    for turn in turns {
        counter = counter+1;
        sumcubes = add_up_cubes(turn);
        println!("turn {}: {}, sum: {}", counter, turn, sumcubes);
    }
    //println!("{}", &gamecontents.next().unwrap());
    
    return sumcubes
    
}

fn add_up_cubes(turn: &str) -> i32 {
    let mut highest_number: Vec<(i32, &str)> = Vec::new();
    let mut sum = 1;

    let sets: Vec<&str> = turn.split(", ").collect();

    for set in sets{
        let elements: Vec<&str> = set.split_whitespace().collect();
        let color = elements[1];
        let number = elements[0].parse::<i32>().unwrap();
        //println!("{}", set);
        if let Some((existing_number, _)) = highest_number // chatgpt magic
            .iter_mut()
            .find(|(_, c)| *c == color)
        {
            *existing_number = std::cmp::max(*existing_number, number);
        } else {
            highest_number.push((number, color));
        
        }
    }

    for number in &highest_number{
        sum = sum * number.0;
    }
    println!("sum: {}", sum);

    sum

}