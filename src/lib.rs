use std::fs::File;
use std::io::prelude::*;

pub struct SudokuSolver {
	grid: [[[u8; 10]; 9]; 9]
}

impl SudokuSolver {
    pub fn new(input_grid_string: String) -> SudokuSolver {
        let mut grid: [[[u8; 10]; 9]; 9] = [[[0; 10]; 9]; 9];
        let mut ints: Vec<u8> = vec![];

        for cell in input_grid_string.chars() {
            match cell.to_digit(10) {
                Some(number) => ints.push(number as u8),
                None => {}
            }
        }

        for r in 0..9 {
            for c in 0..9 {
                grid[r][c][0] = ints[(r*9) + c];
                for d in 1..10 {
                    if grid[r][c][0] != 0 {
                        grid[r][c][d] = 0; //sets all possible for that cell to 0 if a value was given
                    } else {
                        grid[r][c][d] = d as u8; //fills possible if no value was given
                    }
                }
            }
        }

        SudokuSolver {
            grid: grid,
        }
    }

    pub fn output(&mut self, pretty: bool) {
        let mut file = File::create("output.txt").expect("Couldn't create/open file");

        for r in 0..9 {
            for c in 0..9 {
                write!(file, "{} ", self.grid[r][c][0]).expect("Couldn't write");
                if pretty && (c+1)%3 == 0 {
                    write!(file, "| ").expect("Couldn't write");
                }
            }
            writeln!(file, "").expect("Couldn't write");
            if pretty && (r+1)%3 == 0 {
		    	writeln!(file, "----------------------").expect("Couldn't write");
		    }
        }
    }

    pub fn solve(&mut self) -> u8 {
        if self.is_correct() && !self.is_solved() {
            self.set_cell_loop();
        }
        if !self.is_correct() {
            return 1;
        }
        if self.is_solved() {
            return 0;
        }
        // x and y are the coord. and n is the number possible
        let mut x: usize = 0;
        let mut y: usize = 0;
        let mut n: u8 = 0;
        self.find_least_poss(&mut x, &mut y, &mut n);

        for d in 1..10 {
            if self.grid[x][y][d] != 0 {
                self.grid[x][y][0] = self.grid[x][y][d]; //recursively goes through and checks each possibility. if the puzzle becomes solved it bails out. if not it check the next possibility
                if self.solve() == 0 {
                    return 0;
                }
            }
        }

        return 1;
    }

    fn zone_set(&mut self, v: u8, cr: usize, cc: usize) {
        let mut zr = 0;
        let mut zc = 0;

        //determines what zone the value "v" is in. zr and zc are the coordinates of the starting element of the zone
        if cr <= 2 {
            zr = 0;
            if cc <= 2 {
                zc = 0;
            } else if cc >= 3 && cc <= 5 {
                zc = 3;
            } else if cc >= 6 && cc <= 8 {
                zc = 6;
            }
        } else if cr >= 3 && cr <= 5 {
            zr = 3;
            if cc <= 2 {
                zc = 0;
            } else if cc >= 3 && cc <= 5 {
                zc = 3;
            } else if cc >= 6 && cc <= 8 {
                zc = 6;
            }
        } else if cr >= 6 && cr <= 8 {
            zr = 6;
            if cc <= 2 {
                zc = 0;
            } else if cc >= 3 && cc <= 5 {
                zc = 3;
            } else if cc >= 6 && cc <= 8 {
                zc = 6;
            }
        }
        for r in zr..(zr + 3) {
            for c in zc..(zc + 3) {
                //if the possible value is available then set it to 0
                if self.grid[r][c][v as usize] == v {
                    self.grid[r][c][v as usize] = 0;
                }
            }
        }
    }

    fn row_set(&mut self, v: u8, cr: usize) {
        for c in 0..9 {
            //if the possible value is available then set it to 0
            if self.grid[cr][c][v as usize] == v {
                self.grid[cr][c][v as usize] = 0;
            }
        }
    }

    fn col_set(&mut self, v: u8, cc: usize) {
        for r in 0..9 {
            //if the possible value is available then set it to 0
            if self.grid[r][cc][v as usize] == v {
                self.grid[r][cc][v as usize] = 0;
            }
        }
    }

    fn set_possible(&mut self) {
        let mut v: u8;

        for r in 0..9 {
            for c in 0..9 {
                //runs through the whole grid and sets the possible values for each cell
                if self.grid[r][c][0] != 0 {
                    v = self.grid[r][c][0];
                    self.col_set(v, c);
                    self.row_set(v, r);
                    self.zone_set(v, r, c);
                }
            }
        }
    }

    fn set_cell_loop(&mut self) {
        let mut count = 0; //counts possible
        let mut sd: u8 = 0; //singles possibility var.
        let mut changed = true;
        let mut tmp: [u8; 9]; //array of possible values for the cells in a zone/row/column
        let mut zr: usize = 0;
        let mut zc: usize = 0; //zone row, zone column
        'l: while changed {
            changed = false;
            for r in 0..9 {
                'c: for c in 0..9 {
                    self.set_possible();
                    if self.grid[r][c][0] == 0 {
                        //set singles
                        for d in 1..10 {
                            //if a number is found in the empty cell (cell of value 0), then search through the possibilities and count for every possiblility. also store the coord. of the current possibility found
                            if self.grid[r][c][d] != 0 {
                                count += 1;
                                sd = d as u8;
                            }
                        }
                        //if only one possibility found, use the stored coord. to change the cell value to the possibility
                        if count == 1 {
                            self.grid[r][c][0] = sd;
                            changed = true;
                            continue 'c;
                        } else if count == 0 {
                            break 'l;
                        }

                        count = 0;

                        //for unique
                        tmp = [0; 9];

                        //for zone
                        //determines what zone the cell is in. zr and zc are the coordinates of the starting element of the zone
                        if r <= 2 {
                            zr = 0; //top row
                            if c <= 2 {
                                zc = 0;
                            } else if c >= 3 && c <= 5 {
                                zc = 3;
                            } else if c >= 6 && c <= 8 {
                                zc = 6;
                            }
                        } else if r >= 3 && r <= 5 {
                            zr = 3; //middle row
                            if c <= 2 {
                                zc = 0;
                            } else if c >= 3 && c <= 5 {
                                zc = 3;
                            } else if c >= 6 && c <= 8 {
                                zc = 6;
                            }
                        } else if r >= 6 && r <= 8 {
                            zr = 6; //bottom row
                            if c <= 2 {
                                zc = 0;
                            } else if c >= 3 && c <= 5 {
                                zc = 3;
                            } else if c >= 6 && c <= 8 {
                                zc = 6;
                            }
                        }
                        for cr in zr..(zr + 3) {
                            for cc in zc..(zc + 3) {
                                if cr == r && cc != c && self.grid[cr][cc][0] == 0 {
                                    for cd in 1..10 {
                                        if self.grid[cr][cc][cd] != 0 {
                                            //stores value of possibility into index of tmp. this prevents multiple of same value
                                            tmp[self.grid[cr][cc][cd] as usize - 1] = self.grid[cr][cc][cd];
                                        }
                                    }
                                } else if cr != r && self.grid[cr][cc][0] == 0 {
                                    for cd in 1..10 {
                                        if self.grid[cr][cc][cd] != 0 {
                                            //stores value of possibility into index of tmp. this prevents multiple of same value
                                            tmp[self.grid[cr][cc][cd] as usize - 1] = self.grid[cr][cc][cd];
                                        }
                                    }
                                }
                            }
                        }
                        for d in 1..10 {
                            //if a possible value of the cell is unique to the zone, then set the cell to that value
                            if self.grid[r][c][d] != 0 && self.grid[r][c][d] != tmp[self.grid[r][c][d] as usize - 1] {
                                self.grid[r][c][0] = self.grid[r][c][d];
                                changed = true;
                                continue 'c;
                            }
                        }

                        tmp = [0; 9];

                        //for row
                        for cc in 0..9 {
                            if cc != c && self.grid[r][cc][0] == 0 {
                                for cd in 1..10 {
                                    if self.grid[r][cc][cd] != 0 {
                                        tmp[self.grid[r][cc][cd] as usize - 1] = self.grid[r][cc][cd];
                                    }
                                }
                            }
                        }
                        for d in 1..10 {
                            //if a possible value of the cell is unique to the row, then set the cell to that value
                            if self.grid[r][c][d] != 0 && self.grid[r][c][d] != tmp[self.grid[r][c][d] as usize - 1] {
                                self.grid[r][c][0] = self.grid[r][c][d];
                                changed = true;
                                continue 'c;
                            }
                        }

                        tmp = [0; 9];

                        //for col
                        for cr in 0..9 {
                            if cr != r && self.grid[cr][c][0] == 0 {
                                for cd in 1..10 {
                                    if self.grid[cr][c][cd] != 0 {
                                        tmp[self.grid[cr][c][cd] as usize - 1] = self.grid[cr][c][cd];
                                    }
                                }
                            }
                        }

                        for d in 1..10 {
                            //if a possible value of the cell is unique to the column, then set the cell to that value
                            if self.grid[r][c][d] != 0 && self.grid[r][c][d] != tmp[self.grid[r][c][d] as usize - 1] {
                                self.grid[r][c][0] = self.grid[r][c][d];
                                changed = true;
                                continue 'c;
                            }
                        }
                    }
                }
            }
        }

    }

    fn is_solved(&mut self) -> bool {
        for r in 0..9 {
            for c in 0..9 {
                if self.grid[r][c][0] == 0 {
                    return false;
                }
            }
        }

        return true;
    }

    fn is_correct(&mut self) -> bool {
        let mut count = 0;

        for r in 0..9 {
            for c in 0..9 {
                if self.grid[r][c][0] == 0 {
                    for d in 1..10 {
                        if self.grid[r][c][d] == 0 {
                            count += 1;
                        }
                    }

                    if count == 9 {
                        return false;
                    }

                    count = 0;
                }
            }
        }

        return true;
    }

    fn find_least_poss(&mut self, x: &mut usize, y: &mut usize, n: &mut u8) {
        let mut count = 0;
        *n = 9;
        for r in 0..9 {
            for c in 0..9 {
                if self.grid[r][c][0] == 0 {
                    for d in 1..10 {
                        if self.grid[r][c][d] != 0 {
                            count += 1 //counts the number possible
                        }
                    }
                    if count < *n {
                        *n = count; //if the number possible is less than the global possible save it and the coord.
                        *x = r;
                        *y = c;
                    } else {
                        count = 0;
                    }
                }
            }
        }
    }
}




