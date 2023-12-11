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