const INPUT: &str = include_str!("input.txt");

fn check_operators(result: &i128, components: &[i128]) -> bool {
    let max_binary = (1 << (components.len() - 1)) - 1;
    for pattern in 0..=max_binary {
        let mut answer = components[0];
        for (index, component) in (components[1..]).iter().enumerate() {
            if (pattern >> index) & 1 == 1 {
                answer += component;
            } else {
                answer *= component;
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
