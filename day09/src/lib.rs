pub fn interpolate(string: String) -> i32 {
    let mut path_vector: Vec<i32> = string
        .split_whitespace() // split by whitespace (space, tab, newline)
        .filter_map(|s| s.parse().ok()) // convert each substring to an integer
        .collect();

    println!("original vector: \n{:?}", path_vector);
    let mut result_vec = Vec::new();
    vector_difference(&path_vector, &mut result_vec);
    1
}

fn vector_difference(vec: &Vec<i32>, res_vec: &mut Vec<Vec<i32>>) -> Vec<i32> {
    let mut result_vec: Vec<i32> = Vec::new();
    for window in vec.windows(2) {
        let sum = window[1] - window[0];
        result_vec.push(sum);
    }
    println!("{:?}", result_vec);
    result_vec
}
