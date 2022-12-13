use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Read file line by line
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);

    let mut working_path: Vec<String> = vec![String::from("")];
    let mut sizes: HashMap<String, usize> = HashMap::new();

    // Should only be one line
    for line in buf_reader.lines() {
        let line = line.unwrap();

        let mut tokens = line.split(' ');

        match tokens.next().unwrap() {
            "$" => {
                // Command
                let command = tokens.next().unwrap();
                match command {
                    "cd" => {
                        // change dir
                        let path = tokens.next().unwrap();

                        match path {
                            "/" => {
                                working_path = vec![String::from("")];
                            }
                            ".." => {
                                working_path.pop();
                            }
                            other => {
                                working_path.push(String::from(other));
                            }
                        }
                    }
                    "ls" => {
                        // list dir
                        // ignore
                    }
                    other => {
                        // unexpected
                        panic!("unhandled command {}", other);
                    }
                }
            }
            "dir" => {
                // dir
                let _ = tokens.next().unwrap();
            }
            other => {
                // file
                let size: usize = other.parse().unwrap();

                let _file = tokens.next().unwrap();

                for i in 0..working_path.len() {
                    let key = working_path[0..=i].join("/");

                    let en = sizes.entry(key).or_insert(0);
                    *en += size;
                }
            }
        }
    }

    let mut sum = 0;
    for (_, v) in &sizes {
        if *v < 100000 {
            sum += v;
        }
    }
    println!("star1: {}", sum);

    let total_size = sizes.get(&String::from("")).unwrap();
    let delta = *total_size - 40000000;
    let mut smallest = *total_size;
    for (k, v) in &sizes {
        if *v > delta {
            if *v < smallest {
                println!("New smallest: {}", *k);
                smallest = *v;
            }
        }
    }

    println!("star2: {}", smallest);
}
