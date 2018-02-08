extern crate time;
extern crate sudoku_puzzle;
use time::precise_time_s;
use sudoku_puzzle::SudokuSolver;
use std::env;
use std::fs::File;
use std::io::prelude::*;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    // TODO: Add args length check
    if args.len() < 2 {
        panic!("./sudoku_puzzle (your input file)")
    }
    let filename = &args[1];

    let mut file = File::open(filename).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let mut puzzle = SudokuSolver::new(contents);

    let then = precise_time_s();
    let check = puzzle.solve();
    let now = precise_time_s();

	if check == 0 {
        println!("Puzzle solved in {} seconds!", now - then);
        println!("Solved puzzle stored in solved.txt")
	}

    puzzle.output(false);
}