#[derive(Clone)]
struct Queen;

struct Board {
    grid: Vec<Vec<Option<Queen>>>,
}

impl Board {
    fn new(size: usize) -> Board {
        let grid: Vec<Vec<Option<Queen>>> = vec![vec![None; size]; size];
        Board { grid }
    }

    fn to_vec_string(&self) -> Vec<String> {
        let mut board_string: Vec<String> = vec![];

        if self.grid.len() == 0 {
            return board_string;
        }

        for row in 0..self.grid.len() {
            let mut row_string: String = String::new();

            for col in 0..self.grid.len() {
                match self.grid[row][col] {
                    Some(Queen) => row_string.push('Q'),
                    None => row_string.push('.'),
                }
            }
            board_string.push(row_string);
        }

        return board_string;
    }
}

fn within_grid(grid: &Vec<Vec<Option<Queen>>>, row: i8, col: i8) -> bool {
    let within_left_to_right = 0 <= row && row < grid.len() as i8;
    let within_right_to_left = 0 <= col && col < grid.len() as i8;
    return within_left_to_right && within_right_to_left;
}

fn check_diagonal_is_valid(
    grid: &Vec<Vec<Option<Queen>>>,
    row: usize,
    col: usize,
    horizontal: i8,
    vertical: i8,
) -> bool {
    let row = row as i8;
    let col = col as i8;

    let mut new_row: i8 = row + vertical;
    let mut new_col: i8 = col + horizontal;

    while within_grid(&grid, new_row, new_col) {
        match grid[new_row as usize][new_col as usize] {
            Some(Queen) => return false,
            None => {}
        };

        new_row = new_row + vertical;
        new_col = new_col + horizontal;
    }

    true
}

fn is_valid_position(grid: &Vec<Vec<Option<Queen>>>, row: usize, col: usize) -> bool {
    // Check if there is a queen left of current queen
    for position in (0..col).rev() {
        match grid[row][position] {
            Some(Queen) => return false,
            None => {}
        };
    }

    // Check if there is a queen right of the current Queen
    for position in col..grid.len() {
        match grid[row][position] {
            Some(Queen) => return false,
            None => {}
        };
    }

    // Check if there is a queen above of the current Queen
    for position in (0..row).rev() {
        match grid[position][col] {
            Some(Queen) => return false,
            None => {}
        };
    }

    // Check if there is a queen below of the current Queen
    for position in row..grid.len() {
        match grid[position][col] {
            Some(Queen) => return false,
            None => {}
        };
    }

    // left, up
    if !check_diagonal_is_valid(&grid, row, col, -1, 1) {
        return false;
    }

    // left, down
    if !check_diagonal_is_valid(&grid, row, col, -1, -1) {
        return false;
    }

    // right, up
    if !check_diagonal_is_valid(&grid, row, col, 1, 1) {
        return false;
    }

    // right, down
    if !check_diagonal_is_valid(&grid, row, col, 1, -1) {
        return false;
    }

    true
}

fn return_number_of_queens(grid: &Vec<Vec<Option<Queen>>>) -> usize {
    let mut count = 0;

    for row in 0..grid.len() {
        for col in 0..grid.len() {
            match grid[row][col] {
                Some(Queen) => {
                    count = count + 1;
                    continue;
                }
                None => {}
            }
        }
    }

    count
}

fn solve_grid(grid: &mut Vec<Vec<Option<Queen>>>, row: usize, col: usize) -> Vec<Board> {
    if grid.len() == 0 || grid.len() == 0 {
        return vec![];
    }

    if !within_grid(grid, row as i8, col as i8) {
        return vec![];
    }

    let mut possible_boards: Vec<Board> = vec![];

    if is_valid_position(grid, row, col) {
        grid[row][col] = Some(Queen);
        if return_number_of_queens(grid) == grid.len() {
            possible_boards.push(Board {
                grid: grid.to_vec(),
            });
        } else if (row + 1) < grid.len() {
            possible_boards.extend(solve_grid(grid, row + 1, 0));
        }
    }

    grid[row][col] = None;
    if (col + 1) < grid.len() {
        possible_boards.extend(solve_grid(grid, row, col + 1));
    } else if (row + 1) < grid.len() {
        possible_boards.extend(solve_grid(grid, row + 1, 0));
    }

    if row == (grid.len() - 1)
        && col == (grid.len() - 1)
        && return_number_of_queens(grid) == grid.len()
    {
        possible_boards.push(Board {
            grid: grid.to_vec(),
        });
    }

    return possible_boards;
}

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        if n == 0 {
            return vec![];
        }

        let mut board = Board::new(n as usize);
        let mut final_boards: Vec<Board> = vec![];

        final_boards.extend(solve_grid(&mut board.grid, 0, 0));

        let mut final_vec: Vec<Vec<String>> = vec![];

        for board in final_boards.iter() {
            final_vec.push(board.to_vec_string());
        }

        return final_vec;
    }
}

struct Solution;
// Remove below for leetcode
fn main() {
    let wow = Solution::solve_n_queens(4);
    for board in wow.iter() {
        for row in board.iter() {
            println!("[ {} ] ", row);
        }
        println!();
    }
}
