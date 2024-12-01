const INPUT: &str = include_str!("input.txt");

fn push_to_list(list: &mut Vec<i32>, num: &str) {
    let num_int: i32 = num.parse().unwrap();
    list.push(num_int);
}

fn main() {
    let mut first_list: Vec<i32> = Vec::new();
    let mut second_list: Vec<i32> = Vec::new();
    let mut total_diff: i32 = 0;

    for line in INPUT.lines() {
        let mut numbers = line.split_whitespace();

        if let Some(first_num) = numbers.next() {
            push_to_list(&mut first_list, first_num);
        }

        if let Some(second_num) = numbers.next() {
            push_to_list(&mut second_list, second_num);
        }
    }

    first_list.sort();
    second_list.sort();

    for i in 0..first_list.len() {
        total_diff += (first_list[i] - second_list[i]).abs();
    }

    println!("Answer: {}", total_diff);
}
