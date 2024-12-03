const INPUT: &str = include_str!("input.txt");

fn determine_sorted(list: &[i32]) -> bool {
    list.windows(2).all(|w| w[0] < w[1]) || list.windows(2).all(|w| w[0] > w[1])
}

fn is_safe(levels: &[i32]) -> bool {
    for (index, _) in levels.iter().enumerate() {
        let mut valid: bool = true;
        let delta_levels: Vec<i32> = levels
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != index)
            .map(|(_, &x)| x)
            .collect();

        if determine_sorted(&delta_levels) {
            for level_window in delta_levels.windows(2) {
                if (level_window[1] - level_window[0]).abs() > 3 {
                    valid = false;
                    break;
                }
            }
            if valid {
                return valid;
            }
        }
    }
    false
}

fn main() {
    let mut num_safe_reports: i32 = 0;
    for report in INPUT.lines() {
        let levels: Vec<i32> = report
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        if is_safe(&levels) {
            num_safe_reports += 1;
        }
    }

    println!("Answer: {}", num_safe_reports);
}
