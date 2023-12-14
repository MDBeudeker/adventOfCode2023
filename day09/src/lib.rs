pub fn interpolate(string: String) -> (i32, i32) {
    let mut path_vector: Vec<i32> = string
        .split_whitespace() // split by whitespace (space, tab, newline)
        .filter_map(|s| s.parse().ok()) // convert each substring to an integer
        .collect();

    //println!("original vector: \n{:?}", path_vector);
    let mut empty_vec = Vec::new();
    let mut result_vec = vector_difference(&path_vector, &mut empty_vec);
    result_vec.insert(0, path_vector.clone());
    //println! {"new vec is {:?}", result_vec}

    let mut result_vec2 = result_vec.clone();
    result_vec = vector_extrapolate(&mut result_vec);
    result_vec2 = vector_extrapolate_reverse(&mut result_vec2);

    let answer = result_vec[0].last().unwrap();
    let answer2 = result_vec2[0].last().unwrap();
    //println!("answer {}", answer);
    (*answer, *answer2)
}

fn vector_difference(vec: &Vec<i32>, res_vec: &mut Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut new_vec = Vec::new();
    if vec.len() > 1 {
        for window in vec.windows(2) {
            let sum = window[1] - window[0];
            new_vec.push(sum);
        }
        res_vec.push(new_vec.clone());
        if new_vec.iter().all(|&x| x == 0) {
            return res_vec.clone();
        }
        vector_difference(&new_vec, res_vec)
    } else {
        res_vec.clone()
    }
}

fn vector_extrapolate(input_vec: &mut Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut temp_vec: Vec<_> = input_vec.last().cloned().unwrap_or_default();
    let mut new_vec: Vec<Vec<i32>> = vec![];
    let mut zeroes_vec = input_vec.last().cloned().unwrap_or_default();
    zeroes_vec.push(0);
    new_vec.push(zeroes_vec);
    let mut sum = 0;

    for vector in input_vec.windows(2).rev() {
        let old_sum = sum;
        temp_vec = vector[0].clone();
        //sum = vector[1].last().cloned().unwrap_or(0) + vector[0].last().cloned().unwrap_or(0);
        //sum = vector[1].last().cloned().unwrap_or(0) old_sum;
        sum = old_sum + vector[0].last().cloned().unwrap_or(0);
        temp_vec.push(sum);
        new_vec.push(temp_vec.clone());
        //println!("{:?}", temp_vec);
    }

    new_vec.reverse();
    //println!("{:?}", new_vec);
    new_vec
}

fn vector_extrapolate_reverse(input_vec: &mut Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut temp_vec: Vec<_> = input_vec.last().cloned().unwrap_or_default();
    let mut new_vec: Vec<Vec<i32>> = vec![];
    let mut zeroes_vec = input_vec.last().cloned().unwrap_or_default();
    zeroes_vec.push(0);
    new_vec.push(zeroes_vec);
    let mut sum = 0;

    for vector in input_vec.windows(2).rev() {
        let old_sum = sum;
        temp_vec = vector[0].clone();
        temp_vec.reverse();
        //sum = vector[1].last().cloned().unwrap_or(0) - vector[0].last().cloned().unwrap_or(0);
        //sum = vector[1].last().cloned().unwrap_or(0) old_sum;
        sum = -(old_sum - vector[0].first().cloned().unwrap_or(0));
        temp_vec.push(sum);
        println!("{:?}", temp_vec);
        new_vec.push(temp_vec.clone());
        //println!("{:?}", temp_vec);
    }

    new_vec.reverse();
    //println!("{:?}", new_vec);
    new_vec
}
