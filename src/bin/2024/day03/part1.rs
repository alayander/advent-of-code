use regex::Regex;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut product_sum: i32 = 0;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    for cap in re.captures_iter(INPUT) {
        let mut num1: i32 = 0;
        let mut num2: i32 = 0;
        if let Some(cap1) = cap.get(1) {
            num1 = cap1.as_str().parse().unwrap();
        }
        if let Some(cap2) = cap.get(2) {
            num2 = cap2.as_str().parse().unwrap();
        }
        product_sum += num1 * num2;
    }

    println!("Answer: {}", product_sum);
}
