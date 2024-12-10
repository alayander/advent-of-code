const INPUT: &str = include_str!("input.txt");

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

fn traverse(current_location: (usize, usize), direction: Direction, map: &mut Vec<Vec<char>>) {
    map[current_location.0][current_location.1] = 'X';

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
        traverse(current_location, direction.next().unwrap(), map);
    } else {
        traverse(new_location, direction, map);
    }
}

fn main() {
    let mut num_visited: i32 = 0;
    let mut map: Vec<Vec<char>> = INPUT.lines().map(|line| line.chars().collect()).collect();

    let start = find_char(&map, '^');

    traverse(start.unwrap(), Direction::Up, &mut map);

    for row in map {
        for val in row {
            if val == 'X' {
                num_visited += 1;
            }
        }
    }

    println!("Answer: {}", num_visited);
}
