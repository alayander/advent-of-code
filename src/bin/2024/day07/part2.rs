use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, PartialEq)]
enum Operators {
    Add,
    Multiply,
    Concatenate,
}

fn shift_digits(num: &i128) -> i128 {
    (10_i128).pow(num.to_string().len() as u32)
}

fn check_operators(result: &i128, components: &[i128]) -> bool {
    let operators = [Operators::Add, Operators::Multiply, Operators::Concatenate];
    let operator_pool = vec![&operators; components.len() - 1];
    for combination in operator_pool.into_iter().multi_cartesian_product() {
        let mut answer = components[0];
        for (index, component) in (components[1..]).iter().enumerate() {
            match *combination[index] {
                Operators::Add => answer += component,
                Operators::Multiply => answer *= component,
                Operators::Concatenate => answer = answer * shift_digits(component) + component,
            }
        }
        if answer == *result {
            return true;
        }
    }
    false
}

fn main() {
    let mut calibration_result: i128 = 0;

    for line in INPUT.lines() {
        let parts: Vec<i128> = line
            .split(|c: char| c == ':' || c.is_whitespace())
            .filter_map(|s| s.parse::<i128>().ok())
            .collect();

        let result = parts[0];
        let components = &parts[1..];

        if check_operators(&result, components) {
            calibration_result += result;
        }
    }

    println!("Answer: {}", calibration_result);
}
