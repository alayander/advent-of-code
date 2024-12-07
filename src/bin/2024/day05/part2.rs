use std::collections::{HashMap, HashSet, VecDeque};

const INPUT: &str = include_str!("input.txt");

fn topo_sort_middle(update: &Vec<i32>, rule_set: &HashMap<i32, Vec<i32>>) -> i32 {
    let update_set: HashSet<_> = update.iter().collect();
    let mut filtered_rule_set: HashMap<i32, Vec<i32>> = HashMap::new();
    for page in update {
        if let Some(latter_pages) = rule_set.get(page) {
            filtered_rule_set.insert(
                *page,
                latter_pages
                    .iter()
                    .filter(|&p| update_set.contains(p))
                    .cloned()
                    .collect(),
            );
        }
    }

    let mut in_degree: HashMap<i32, i32> = HashMap::new();
    let mut sorted_result: Vec<i32> = Vec::new();
    let mut queue: VecDeque<i32> = VecDeque::new();
    for (&node, edges) in &filtered_rule_set {
        in_degree.entry(node).or_insert(0);
        for &neighbour in edges {
            *in_degree.entry(neighbour).or_insert(0) += 1;
        }
    }

    for (&node, &degree) in &in_degree {
        if degree == 0 {
            queue.push_back(node);
        }
    }

    while let Some(node) = queue.pop_front() {
        sorted_result.push(node);

        if let Some(edges) = filtered_rule_set.get(&node) {
            for &neighbor in edges {
                if let Some(degree) = in_degree.get_mut(&neighbor) {
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    *sorted_result.get(sorted_result.len() / 2).unwrap()
}

fn check_results(update: &Vec<i32>, rule_set: &HashMap<i32, Vec<i32>>) -> i32 {
    for (i, page) in update.iter().enumerate() {
        let latter_pages = rule_set.get(page).unwrap();
        let prev_update_pages = &update[..i];

        let errors = prev_update_pages.iter().any(|&x| latter_pages.contains(&x));
        if errors {
            return topo_sort_middle(update, rule_set);
        }
    }
    0
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
