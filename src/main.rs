use std::env;
use std::fmt::Display;
use std::path::Path;
use std::io;

mod board;

fn process_file<P: AsRef<Path> + Display>(file: P) -> io::Result<bool> {
    // if error send to parent function
	let mut board = board::load(&file)?;

    // solve the board
	if board::solve(&mut board) {
		println!("=====> {} <=====", file);
		board::print(&board);
		println!();
		Ok(true)
	} else {
		Ok(false)
	}
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
		println!("Usage: sudoku <file 1>, <file 2>, ..., <file n>");
		std::process::exit(0);
    };

	for file in args.iter().skip(1) {
		match process_file(file) {
			Ok(false) => println!("Board is unsolvable: {}", file),
			Err(e) => {
				println!("{} -> \"{}\"", e, file);
				println!();
			}
            // pour tout le reste print le board
			_ => {}
		}
	};
}
