use std::fs::File;
use std::io::{BufRead, BufReader};

extern crate ansi_term;
use ansi_term::Colour::Red;

#[derive(Debug, Clone)]
struct Board {
    vals: [usize; 25],
    marks: [bool; 25],
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..5 {
            for j in 0..5 {
                let val = format!("{:02} ", self.vals[i*5+j]);
                if self.marks[i*5+j] {
                    write!(f, "{}", Red.bold().paint(&val)).unwrap();
                } else {
                    write!(f, "{}", val).unwrap();
                }

            }
            write!(f, "\n").unwrap();
        }
        Ok(())
    }
}

impl Board {
    fn is_winner(&self) -> bool {
        let mut down_diag_solved = true;
        let mut up_diag_solved = true;
        for i in 0..5 {
            let mut row_solved = true;
            let mut col_solved = true;
            for j in 0..5 {
                row_solved = row_solved && self.marks[i*5 + j];
                col_solved = col_solved && self.marks[j*5 + i];
            }
            if row_solved || col_solved { return true }

            up_diag_solved = up_diag_solved && self.marks[((4-i)*5)+i];
            down_diag_solved = down_diag_solved && self.marks[(i*5)+i];
        }

        //if up_diag_solved || down_diag_solved { return true }

        false
    }

    fn idx_of(&self, n: usize) -> Result<usize, ()> {
        for i in 0..25 {
            if self.vals[i] == n {
                return Ok(i);
            }
        }
        Err(())
    }

    fn score(&self) -> usize {
        let mut sum = 0;
        for i in 0..25 {
            if !self.marks[i] {
                sum += self.vals[i];
            }
        }

        sum
    }

    fn mark_board(&mut self, number: usize) -> usize {
        match self.idx_of(number) {
            Ok(idx) => { self.marks[idx] = true; }
            Err(_) => {}
        }

        if self.is_winner() {
            return self.score() * number;
        }

        0
    }

    fn win(&mut self, numbers: &Vec<usize>) -> (usize, usize) {
        for (i, n) in numbers.iter().copied().enumerate() {
            let score = self.mark_board(n);
            if score > 0 {
                return (i, score)
            }
        }
        (0,0)
    }

    fn new(vals: Vec<usize>) -> Board {
        return Board{
            vals: vals.try_into().unwrap(),
            marks: [false; 25],
        }
    }
}

fn main() {
    // Read file line by line
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);

    let mut lines = buf_reader.lines();

    let numbers_str = lines.next().unwrap().unwrap();

    let numbers: Vec<usize> = numbers_str.split(",").map(|x| { x.parse().unwrap() }).collect();

    lines.next();

    let mut boards = Vec::new();
    let mut board_vals = Vec::new();
    for line in lines {
        let s = line.unwrap();
        if s.len() == 0 {
            let board = Board::new(board_vals);
            boards.push(board);
            board_vals = Vec::new();
            continue;
        }
        for n in s.split_whitespace() {
            let n = n.parse().unwrap();
            board_vals.push(n);
        }
    }

    // Part1
    for n in &numbers {
        let mut max_score = 0;
        for b in boards.iter_mut() {
            let score = b.mark_board(*n);
            if score > max_score {
                //println!("Board");
                //println!("{}", b);
                max_score = score;
            }
        }
        if max_score > 0 {
            println!("Part1: First board won");
            println!("Done after {}: {}", n, max_score);
            break;
        }
    }

    // Part2
    let mut max_depth = 0;
    let mut deepest_score = 0;
    for mut b in boards.clone() {
        let (d, s) = b.win(&numbers);
        if d > max_depth {
            max_depth = d;
            deepest_score = s;
        }
    }
    println!("Part2: Last board won");
    println!("Last board scored after {}: {}", numbers[max_depth], deepest_score);
}
