const INPUT: &str = include_str!("input.txt");

fn determine_sorted(list: &[i32]) -> bool {
    list.windows(2).all(|w| w[0] < w[1]) || list.windows(2).all(|w| w[0] > w[1])
}

fn main() {
    let mut num_safe_reports: i32 = 0;
    for report in INPUT.lines() {
        let mut valid: bool = true;
        let levels: Vec<i32> = report
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        if determine_sorted(&levels) {
            for level_window in levels.windows(2) {
                if (level_window[1] - level_window[0]).abs() > 3 {
                    valid = false;
                    break;
                }
            }
            if valid {
                num_safe_reports += 1;
            }
        }
    }

    println!("Answer: {}", num_safe_reports);
}
