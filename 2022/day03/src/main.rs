use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Read file line by line
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);

    let mut sum = 0;
    for lin in buf_reader.lines() {
        let line = String::from(lin.unwrap());
        let pivot = line.len() / 2;
        let part1 = &line[0..pivot];
        let part2 = &line[pivot..line.len()];

        sum += common(&[&part1, &part2]);
    }
    println!("star1: {}", sum);

    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(&file);

    let mut lines = buf_reader.lines();

    let mut sum = 0;
    loop {
        let a = match lines.next() {
            Some(l) => l.unwrap(),
            None => {
                break;
            }
        };
        let b = lines.next().unwrap().unwrap();
        let c = lines.next().unwrap().unwrap();

        sum += common(&[&a, &b, &c]);
    }

    println!("star2: {}", sum);
}

fn rank(c: char) -> u32 {
    if c.is_ascii_uppercase() {
        return 27 + u32::from(c) - 65;
    }

    if c.is_ascii_lowercase() {
        return 1 + u32::from(c) - 97;
    }

    return 0;
}

fn string_to_encoded(s: &str) -> u64 {
    let mut letters = 0;
    for c in s.chars() {
        letters = letters | (1 << (rank(c) - 1));
    }
    return letters;
}

fn common(strs: &[&str]) -> u32 {
    let v = strs
        .iter()
        .map(|s| string_to_encoded(s))
        .reduce(|a, b| a & b)
        .unwrap();

    for i in 0..64 {
        if (v & (1 << i)) > 0 {
            return i + 1;
        }
    }
    return 0;
}
