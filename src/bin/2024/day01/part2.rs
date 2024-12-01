use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

fn push_to_hashmap(hashmap: &mut HashMap<i32, i32>, num: &str) {
    let num_int: i32 = num.parse().unwrap();
    *hashmap.entry(num_int).or_insert(0) += 1;
}

fn main() {
    let mut left_counts: HashMap<i32, i32> = HashMap::new();
    let mut right_counts: HashMap<i32, i32> = HashMap::new();
    let mut similarity_score: i32 = 0;

    for line in INPUT.lines() {
        let mut numbers = line.split_whitespace();

        if let Some(first_num) = numbers.next() {
            push_to_hashmap(&mut left_counts, first_num);
        }

        if let Some(second_num) = numbers.next() {
            push_to_hashmap(&mut right_counts, second_num);
        }
    }

    for key in left_counts.keys() {
        similarity_score +=
            key * left_counts.get(key).unwrap() * right_counts.get(key).unwrap_or(&0);
    }

    println!("Answer: {}", similarity_score);
}
