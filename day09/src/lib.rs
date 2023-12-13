pub fn interpolate(string: String) -> i32 {
    let mut path_vector: Vec<i32> = string
        .split_whitespace() // split by whitespace (space, tab, newline)
        .filter_map(|s| s.parse().ok()) // convert each substring to an integer
        .collect();

    println!("original vector: \n{:?}", path_vector);
    let mut empty_vec = Vec::new();
    let mut result_vec = vector_difference(&path_vector, &mut empty_vec);
    result_vec.insert(0, path_vector.clone());
    println! {"new vec is {:?}", result_vec}
    result_vec = vector_extrapolate(&mut result_vec);

    1
}

fn vector_difference(vec: &Vec<i32>, res_vec: &mut Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut new_vec = Vec::new();
    if vec.len() > 1 {
        for window in vec.windows(2) {
            let sum = window[1] - window[0];
            new_vec.push(sum);
        }
        println!("{:?}", new_vec);
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
    let mut new_vec = Vec::new();
    for vector in input_vec.iter().rev() {
        new_vec.push(vector);
        println!("{:?}", new_vec);
    }
    vec![vec![1], vec![2]]
}
