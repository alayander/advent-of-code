const INPUT: &str = include_str!("input.txt");

fn x_mas_check(row: &usize, col: &usize, grid: &[Vec<char>]) -> bool {
    if ((grid[row - 1][col - 1] == 'M' && grid[row + 1][col + 1] == 'S')
        || (grid[row - 1][col - 1] == 'S' && grid[row + 1][col + 1] == 'M'))
        && ((grid[row - 1][col + 1] == 'M' && grid[row + 1][col - 1] == 'S')
            || (grid[row - 1][col + 1] == 'S' && grid[row + 1][col - 1] == 'M'))
    {
        return true;
    }
    false
}

fn main() {
    let mut xmas_count: i32 = 0;
    let letter_grid: Vec<Vec<char>> = INPUT.lines().map(|line| line.chars().collect()).collect();

    for (row_num, row) in letter_grid.iter().enumerate() {
        for (col_num, letter) in row.iter().enumerate() {
            if *letter == 'A' {
                if row_num == 0
                    || row_num == letter_grid.len() - 1
                    || col_num == 0
                    || col_num == row.len() - 1
                {
                    continue;
                }

                if x_mas_check(&row_num, &col_num, &letter_grid) {
                    xmas_count += 1;
                }
            }
        }
    }

    println!("Answer: {}", xmas_count);
}
