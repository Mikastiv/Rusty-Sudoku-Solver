use std::fs;
use std::io::{self, Read};
use std::path::Path;

pub use 
solver::solve;

mod solver;

const BOARD_SIZE: usize = 81;

pub fn load<P: AsRef<Path>>(file: P) -> io::Result<Vec<u32>> {
    let f = fs::File::open(file)?;
    let mut board = Vec::with_capacity(BOARD_SIZE);

    for c in f.bytes() {
        let c = char::from(c?);
        if !c.is_ascii_digit() {
            continue;
        }
        board.push(c.to_digit(10).unwrap());
    }

    if board.len() != BOARD_SIZE {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "Board size incorrect, should be {}, but is {}",
                BOARD_SIZE,
                board.len()
            ),
        ));
    }

    Ok(board)
}

pub fn print(board: &[u32]) {
    for row in 0..9 {
        if row % 3 == 0 {
            println!("-------------------------");
        }
        for col in 0..9 {
            if col % 3 == 0 {
                print!("| ");
            }
            print!("{} ", board[get_1d_index(row, col)]);
        }
        println!("|");
    }
    println!("-------------------------");
}

pub fn get_1d_index(row: usize, col: usize) -> usize {
    row * 9 + col
}
