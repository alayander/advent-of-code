const INPUT: &str = include_str!("input.txt");

const XMAS_LETTERS: [char; 4] = ['X', 'M', 'A', 'S'];

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

fn xmas_check(
    row: &usize,
    col: &usize,
    grid: &Vec<Vec<char>>,
    direction: &Direction,
    letter_index: &i32,
) -> bool {
    let mut new_coord = [*row, *col];
    match direction {
        Direction::Up => new_coord[0] -= 1,
        Direction::Down => new_coord[0] += 1,
        Direction::Left => new_coord[1] -= 1,
        Direction::Right => new_coord[1] += 1,
        Direction::UpLeft => {
            new_coord[0] -= 1;
            new_coord[1] -= 1;
        }
        Direction::UpRight => {
            new_coord[0] -= 1;
            new_coord[1] += 1;
        }
        Direction::DownLeft => {
            new_coord[0] += 1;
            new_coord[1] -= 1;
        }
        Direction::DownRight => {
            new_coord[0] += 1;
            new_coord[1] += 1;
        }
    }
    if grid[new_coord[0]][new_coord[1]] == XMAS_LETTERS[(*letter_index + 1) as usize] {
        if *letter_index + 1 == (XMAS_LETTERS.len() as i32) - 1 {
            return true;
        } else {
            return xmas_check(
                &new_coord[0],
                &new_coord[1],
                grid,
                direction,
                &(*letter_index + 1),
            );
        }
    }
    false
}

fn main() {
    let mut xmas_count: i32 = 0;
    let possible_directions: Vec<Direction> = vec![
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
        Direction::UpLeft,
        Direction::UpRight,
        Direction::DownLeft,
        Direction::DownRight,
    ];
    let letter_grid: Vec<Vec<char>> = INPUT.lines().map(|line| line.chars().collect()).collect();

    for (row_num, row) in letter_grid.iter().enumerate() {
        for (col_num, letter) in row.iter().enumerate() {
            if *letter == XMAS_LETTERS[0] {
                let mut directions: Vec<Direction> = possible_directions.clone();

                if row_num < 3 {
                    directions.retain(|dir| {
                        *dir != Direction::Up
                            && *dir != Direction::UpLeft
                            && *dir != Direction::UpRight
                    });
                } else if row_num >= letter_grid.len() - 3 {
                    directions.retain(|dir| {
                        *dir != Direction::Down
                            && *dir != Direction::DownLeft
                            && *dir != Direction::DownRight
                    });
                }

                if col_num < 3 {
                    directions.retain(|dir| {
                        *dir != Direction::Left
                            && *dir != Direction::UpLeft
                            && *dir != Direction::DownLeft
                    });
                } else if col_num >= row.len() - 3 {
                    directions.retain(|dir| {
                        *dir != Direction::Right
                            && *dir != Direction::UpRight
                            && *dir != Direction::DownRight
                    });
                }

                for dir in directions {
                    if xmas_check(&row_num, &col_num, &letter_grid, &dir, &0) {
                        xmas_count += 1;
                    }
                }
            }
        }
    }

    println!("Answer: {}", xmas_count);
}
