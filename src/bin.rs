extern crate sudoku_puzzle;
use sudoku_puzzle::SudokuSolver;
use std::env;
use std::fs::File;
use std::io::prelude::*;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    // TODO: Add args length check
    let filename = &args[1];

    let mut file = File::open(filename).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let puzzle = SudokuSolver::new(contents);

    puzzle.output(true);
}