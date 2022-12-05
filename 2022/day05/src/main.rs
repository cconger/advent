use std::fs::File;
use std::io::{BufRead, BufReader};

#[macro_use]
extern crate scan_fmt;

fn print_stacks(v: &Vec<Vec<char>>) {
    let max_height = v.iter().map(|s| s.len()).max().unwrap();

    for i in 1..=max_height {
        for s in v {
            print!("[{}] ", s.get(max_height - i).unwrap_or(&' '));
        }
        print!("\n");
    }

    for (i, _) in v.iter().enumerate() {
        print!(" {}  ", i);
    }
    print!("\n");
}

fn main() {
    // Read file line by line
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);

    let mut stacks: Vec<Vec<char>> = vec![];

    let mut lines = buf_reader.lines().into_iter();

    'parse: for lin in &mut lines {
        let line = lin.unwrap();

        let v = line.chars().skip(1).step_by(4);
        for (i, c) in v.enumerate() {
            if stacks.len() <= i {
                stacks.push(vec![]);
            }

            if c == ' ' {
                continue;
            }
            if c.is_ascii_digit() {
                for s in &mut stacks {
                    s.reverse();
                }
                break 'parse;
            }

            stacks[i].push(c);
        }
    }

    lines.next();

    for lin in lines {
        if let Ok((count, from, to)) = scan_fmt!(
            &lin.unwrap(),
            "move {d} from {d} to {d}",
            usize,
            usize,
            usize
        ) {
            let mut tmp: Vec<char> = vec![];

            // Use a temporary stack
            for _ in 0..count {
                let hold = stacks[from - 1].pop().unwrap();
                tmp.push(hold);
            }
            for _ in 0..count {
                let hold = tmp.pop().unwrap();
                stacks[to - 1].push(hold);
            }

            // Solution for star1
            // for _ in 0..count {
            //     let hold = stacks[from - 1].pop().unwrap();
            //     stacks[to - 1].push(hold);
            // }
        }
    }

    let out: String = stacks.iter().map(|s| s.last().unwrap()).collect();

    println!("star2: {}", out);
}
