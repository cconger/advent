use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;

#[macro_use]
extern crate scan_fmt;

enum Operation {
    Noop,
    Add(isize),
}

fn main() {
    // Read file line by line
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);

    let sample_ticks = [20, 60, 100, 140, 180, 220];
    let mut samples = [0, 0, 0, 0, 0, 0];
    let mut clock = 1;

    let mut next_sample_iter = sample_ticks.iter().enumerate();
    let mut next_sample = next_sample_iter.next().unwrap();
    let mut x = 1;

    let mut crt: Vec<char> = Vec::with_capacity(240);

    for line in buf_reader.lines() {
        let (op, v) = scan_fmt_some!(&line.unwrap(), "{} {d}", String, isize);
        let op = match op.unwrap().as_str() {
            "noop" => Operation::Noop,
            "addx" => Operation::Add(v.unwrap()),
            _ => panic!("unexpected input"),
        };

        let clocks = match op {
            Operation::Noop => 1,
            Operation::Add(_) => 2,
        };

        for _ in 0..clocks {
            if clock % 40 >= x && clock % 40 < x + 3 {
                crt.push('#');
            } else {
                crt.push('.');
            }
            clock += 1;
        }

        //clock += clocks;
        if clock > *next_sample.1 {
            samples[next_sample.0] = x;
            next_sample = match next_sample_iter.next() {
                Some(s) => s,
                None => (8, &10000),
            }
        }

        match op {
            Operation::Noop => {}
            Operation::Add(v) => {
                x += v;
            }
        }
    }

    let signal: isize = zip(sample_ticks, samples).map(|(t, s)| t * s).sum();
    println!("{:?} {:?}", sample_ticks, samples);
    println!("star1: {}", signal);

    println!("star2:");
    for (i, c) in crt.iter().enumerate() {
        print!("{}", c);
        if (i + 1) % 40 == 0 {
            print!("\n");
        }
    }
}
