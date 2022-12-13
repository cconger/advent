use std::fs::File;
use std::io::{BufRead, BufReader};

enum Play {
    Rock,     // A X
    Paper,    // B Y
    Scissors, // C Z
}

fn main() {
    // Read file line by line
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);

    let mut sum = 0;
    for lin in buf_reader.lines() {
        match lin {
            Ok(s) => {
                let score = match &s as &str {
                    "A X" => 3,
                    "B X" => 1,
                    "C X" => 2,
                    "A Y" => 4,
                    "B Y" => 5,
                    "C Y" => 6,
                    "A Z" => 8,
                    "B Z" => 9,
                    "C Z" => 7,
                    _ => {
                        println!("Unexpected");
                        0
                    }
                };
                sum += score;
            }
            Err(e) => {
                println!("ohno");
            }
        }
    }

    println!("Star2: {}", sum);
}
