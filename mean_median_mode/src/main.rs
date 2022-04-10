use std::collections::HashMap;

fn main() {
    let mut int_vec = vec![4, 1];

    println!("Vec: {:?}", int_vec);

    println!("Mean: {}", calculate_mean(&int_vec));
    println!("Median: {}", calculate_median(&mut int_vec));
    println!("Mode: {:?}", calculate_mode(&int_vec));
}

fn calculate_mean(input: &Vec<i32>) -> f32 {
    let mut total = 0.0;

    for i in input {
        total += *i as f32;
    }

    total / input.len() as f32
}

fn calculate_median(input: &mut Vec<i32>) -> f32 {
    input.sort();
    let i = input.len() / 2;

    if input.len() % 2 == 1 {
        input[i] as f32
    } else {
        (input[i - 1] as f32 + input[i] as f32) / 2.0
    }
}

fn calculate_mode(input: &Vec<i32>) -> Vec<i32> {
    // count occurences of ints and store counts in a hashmap
    let mut int_freq = HashMap::new();
    for int in input {
        let count = int_freq.entry(int).or_insert(0);
        *count += 1;
    }

    // get maximum value from hashmap
    let max = int_freq.values().map(|v| *v).max().unwrap();

    // get every key with a value == max
    let mut vec = int_freq
        .into_iter()
        .filter_map(|(k, v)| if v == max { Some(*k) } else { None })
        .collect::<Vec<i32>>();

    vec.sort();
    vec
}
