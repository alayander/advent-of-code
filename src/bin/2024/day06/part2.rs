use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next(self) -> Option<Direction> {
        match self {
            Direction::Up => Some(Direction::Right),
            Direction::Right => Some(Direction::Down),
            Direction::Down => Some(Direction::Left),
            Direction::Left => Some(Direction::Up),
        }
    }
}

fn find_char(matrix: &[Vec<char>], target: char) -> Option<(usize, usize)> {
    for (i, row) in matrix.iter().enumerate() {
        if let Some(j) = row.iter().position(|&c| c == target) {
            return Some((i, j));
        }
    }
    None
}

fn check_loop(current_location: (usize, usize), direction: &Direction, map: &[Vec<char>]) -> bool {
    let mut guard_position: (usize, usize) = current_location;
    let mut guard_direction: Direction = direction.clone();
    let mut visited: Vec<((usize, usize), Direction)> = Vec::new();

    loop {
        let mut new_location_i32: (i32, i32) = (guard_position.0 as i32, guard_position.1 as i32);
        match guard_direction {
            Direction::Up => new_location_i32.0 -= 1,
            Direction::Down => new_location_i32.0 += 1,
            Direction::Left => new_location_i32.1 -= 1,
            Direction::Right => new_location_i32.1 += 1,
        }
        if new_location_i32.0 < 0
            || new_location_i32.0 >= map[0].len() as i32
            || new_location_i32.1 < 0
            || new_location_i32.1 >= map.len() as i32
        {
            return false;
        }

        let new_location: (usize, usize) =
            (new_location_i32.0 as usize, new_location_i32.1 as usize);
        if map[new_location.0][new_location.1] == '#' {
            guard_direction = guard_direction.next().unwrap();
            let guard_info = (guard_position, guard_direction.clone());
            if visited.iter().any(|item| *item == guard_info) {
                return true;
            }
            visited.push(guard_info);
        } else {
            guard_position = new_location;
        }
    }
}

fn traverse(
    start_location: (usize, usize),
    current_location: (usize, usize),
    direction: Direction,
    map: &mut Vec<Vec<char>>,
    loop_obstacle_set: &mut HashSet<(usize, usize)>,
) {
    let mut new_location_i32: (i32, i32) = (current_location.0 as i32, current_location.1 as i32);
    match direction {
        Direction::Up => new_location_i32.0 -= 1,
        Direction::Down => new_location_i32.0 += 1,
        Direction::Left => new_location_i32.1 -= 1,
        Direction::Right => new_location_i32.1 += 1,
    }

    if new_location_i32.0 < 0
        || new_location_i32.0 >= map[0].len() as i32
        || new_location_i32.1 < 0
        || new_location_i32.1 >= map.len() as i32
    {
        return;
    }

    let new_location: (usize, usize) = (new_location_i32.0 as usize, new_location_i32.1 as usize);
    if map[new_location.0][new_location.1] == '#' {
        traverse(
            start_location,
            current_location,
            direction.next().unwrap(),
            map,
            loop_obstacle_set,
        );
    } else {
        let mut test_loop_map = map.clone();
        if new_location != start_location {
            test_loop_map[new_location.0][new_location.1] = '#';
            if check_loop(start_location, &Direction::Up, &test_loop_map) {
                loop_obstacle_set.insert(new_location);
            }
        }
        traverse(
            start_location,
            new_location,
            direction,
            map,
            loop_obstacle_set,
        );
    }
}

fn main() {
    let mut map: Vec<Vec<char>> = INPUT.lines().map(|line| line.chars().collect()).collect();
    let mut loop_obstacle_set: HashSet<(usize, usize)> = HashSet::new();

    let start = find_char(&map, '^');

    traverse(
        start.unwrap(),
        start.unwrap(),
        Direction::Up,
        &mut map,
        &mut loop_obstacle_set,
    );
    let num_looping_obstacles = loop_obstacle_set.len() as i32;

    println!("Answer: {}", num_looping_obstacles);
}
