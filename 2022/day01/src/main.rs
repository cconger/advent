use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Read file line by line
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);

    let mut vals = vec![];
    let mut sum = 0;
    for line in buf_reader.lines() {
        match line {
            Ok(str) => {
                if str.len() == 0 {
                    vals.push(sum);
                    sum = 0;
                    continue;
                }
                let v: i64 = str.parse().unwrap();
                sum += v;
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }

    vals.sort_by(|a, b| b.cmp(a));

    println!("Star1: {}", vals[0]);
    println!("Star2: {}", vals[0..=2].iter().sum::<i64>());
}
