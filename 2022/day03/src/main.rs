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

        sum += common_letters(&[&part1, &part2]);
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

        sum += common_letters(&[&a, &b, &c]);
    }

    println!("star2: {}", sum);
}

fn letter_to_score(c: char) -> u32 {
    if c.is_ascii_uppercase() {
        return 27 + u32::from(c) - 65;
    }

    if c.is_ascii_lowercase() {
        return 1 + u32::from(c) - 97;
    }

    return 0;
}

fn common_letters(strs: &[&str]) -> u32 {
    let mut count: [usize; 52] = [0; 52];

    for s in strs {
        let mut letters: [usize; 52] = [0; 52];
        for c in s.chars() {
            letters[letter_to_score(c) as usize - 1] = 1;
        }
        for (i, c) in letters.iter().enumerate() {
            count[i] += c;
        }
    }

    for (i, c) in count.iter().enumerate() {
        if *c == strs.len() {
            return i as u32 + 1;
        }
    }
    println!("UHOH");
    return 0;
}
