use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Read file line by line
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);

    let mut corrections: Vec<u64> = Vec::new();

    let mut error_sum = 0;
    for line in buf_reader.lines() {
        let line = line.unwrap();
        let mut line_err = 0;
        let mut stack: Vec<u8> = Vec::new();

        for c in line.chars() {
            match c {
                '[' => { stack.push(2); }
                ']' => {
                    if *stack.last().unwrap() != 2 {
                        line_err = 57;
                        break;
                    } else {
                        stack.pop();
                    }
                },
                '(' => { stack.push(1); }
                ')' => {
                    if *stack.last().unwrap() != 1 {
                        line_err = 3;
                        break;
                    } else {
                        stack.pop();
                    }
                },
                '{' => { stack.push(3); },
                '}' => {
                    if *stack.last().unwrap() != 3 {
                        line_err = 1197;
                        break;
                    } else {
                        stack.pop();
                    }
                },
                '<' => { stack.push(4); },
                '>' => {
                    if *stack.last().unwrap() != 4 {
                        line_err = 25137;
                        break;
                    } else {
                        stack.pop();
                    }
                },
                _ => {
                    println!("Unexpected symbol: {}", c);
                    return
                }
            }
        }

        if line_err > 0 {
            error_sum += line_err;
        } else {
            let correction = stack.iter().rev().fold(0, |acc, x| { acc * 5 + *x as u64 });
            corrections.push(correction);
        }
    }

    corrections.sort();

    println!("Part1: Sum of errors on corrupted lines: {}", error_sum);
    println!("Part2: Median Correction: {:?}", corrections[corrections.len() / 2]);
}
