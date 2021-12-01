use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Read file line by line
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);

    let values: Vec<i64> = buf_reader.lines().map(|x| { x.unwrap().parse().unwrap() }).collect();

    let mut count: i64 = 0;
    for i in 3..values.len() {
        if values[i] > values[i-3] {
            count += 1;
        }
    }

    println!("Increasing: {}", count)
}
