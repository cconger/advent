use std::fs::File;
use std::io::{BufRead, BufReader};

fn first_unique(chars: &Vec<char>, len: usize) -> usize {
    'window: for idx in len..chars.len() {
        let slice = &chars[idx - len..idx];

        // Thought about using IterTools windowing on a chars iter
        for (i, c) in slice.iter().enumerate() {
            for j in i + 1..slice.len() {
                if *c == slice[j] {
                    continue 'window;
                }
            }
        }
        return idx;
    }

    return 0;
}

fn main() {
    // Read file line by line
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);

    // Should only be one line
    for line in buf_reader.lines() {
        let line = line.unwrap();

        let chars: Vec<char> = line.chars().collect();

        println!("star1: {}", first_unique(&chars, 4));
        println!("star2: {}", first_unique(&chars, 14));
    }
}
