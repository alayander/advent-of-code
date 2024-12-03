use regex::Regex;
use std::collections::BTreeMap;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut product_sum: i32 = 0;
    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let do_dont_re = Regex::new(r"do\(\)|don't\(\)").unwrap();

    let mut do_dont_map: BTreeMap<i32, bool> = BTreeMap::new();
    do_dont_map.insert(-1, true);

    for mat in do_dont_re.find_iter(INPUT) {
        let matched = mat.as_str();
        let value: bool = matched == "do()";

        if let Some((_, last_value)) = do_dont_map.iter().last() {
            if *last_value != value {
                do_dont_map.insert(mat.start() as i32, value);
            }
        }
    }

    for cap in mul_re.captures_iter(INPUT) {
        let index: i32 = cap.get(0).map(|mat| mat.start()).unwrap_or(0) as i32;
        let mut num1: i32 = 0;
        let mut num2: i32 = 0;

        let prev_cmd = do_dont_map.range(..index).next_back();
        if prev_cmd.map(|(_, b)| *b).unwrap() {
            if let Some(cap1) = cap.get(1) {
                num1 = cap1.as_str().parse().unwrap();
            }
            if let Some(cap2) = cap.get(2) {
                num2 = cap2.as_str().parse().unwrap();
            }
            product_sum += num1 * num2;
        }
    }

    println!("Answer: {}", product_sum);
}
