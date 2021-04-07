use super::get_1d_index;

fn row_valid(row: usize, number: u32, board: &[u32]) -> bool {
    for col in 0..9 {
        if board[get_1d_index(row, col)] == number {
            return false;
        }
    }

    true
}

fn col_valid(col: usize, number: u32, board: &[u32]) -> bool {
    for row in 0..9 {
        if board[get_1d_index(row, col)] == number {
            return false;
        }
    }
    true
}

fn square_valid(row: usize, col: usize, number: u32, board: &[u32]) -> bool {
    let square_row = row / 3;
    let square_col = col / 3;

    for row in (square_row * 3)..(square_row * 3 + 3) {
        if board[get_1d_index(row, col)] == number {
            return false;
        }
        for col in (square_col * 3)..(square_col * 3 + 3) {
            if board[get_1d_index(row, col)] == number {
                return false;
            }
        }
    }

    true
}

fn num_valid(row: usize, col: usize, number: u32, board: &[u32]) -> bool {
    row_valid(row, number, board)
        && col_valid(col, number, board)
        && square_valid(row, col, number, board)
}

fn next_num(board: &[u32]) -> (usize, usize) {
    for row in 0..9 {
        for col in 0..9 {
            if board[get_1d_index(row, col)] == 0 {
                return (row, col);
            }
        }
    }

    (usize::MAX, usize::MAX)
}

pub fn solve(board: &mut [u32]) -> bool {
    // Find coordinates of the next number to try
    let (row, col) = next_num(board);
    // If no empty slot found, sudoku solved !
    if row == usize::MAX && col == usize::MAX {
        return true;
    }

    for num in 1..=9 {
        if num_valid(row, col, num, &board) {
            // Put num in board
            board[get_1d_index(row, col)] = num;
            // Recurse with new state of board
            if solve(board) {
                return true;
            }
            // Put back 0 if backtraced to here
            board[get_1d_index(row, col)] = 0;
        }
    }

    false
}
