use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

fn check_results(update: &[i32], rule_set: &HashMap<i32, Vec<i32>>) -> i32 {
    for (i, page) in update.iter().enumerate() {
        let latter_pages = rule_set.get(page).unwrap();
        let prev_update_pages = &update[..i];

        let errors = prev_update_pages.iter().any(|&x| latter_pages.contains(&x));
        if errors {
            return 0;
        }
    }
    *update.get(update.len() / 2).unwrap()
}

fn main() {
    let mut correct_middle_sum: i32 = 0;
    let mut rule_set: HashMap<i32, Vec<i32>> = HashMap::new();

    let mut groups = INPUT.split("\n\n");
    let rules_input = groups.next().unwrap_or("");
    let updates_input = groups.next().unwrap_or("");

    for rule in rules_input.lines() {
        let entry: Vec<&str> = rule.split('|').collect();
        rule_set
            .entry(entry[0].parse::<i32>().unwrap())
            .or_default()
            .push(entry[1].parse::<i32>().unwrap());
    }

    for update in updates_input.lines() {
        let update_vec: Vec<i32> = update
            .split(',')
            .filter_map(|s| s.trim().parse::<i32>().ok())
            .collect();
        correct_middle_sum += check_results(&update_vec, &rule_set);
    }

    println!("Answer: {}", correct_middle_sum);
}
