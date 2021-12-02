use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1(buf_reader: BufReader<File>) {
    // Part 1
    let mut depth = 0;
    let mut pos = 0;
    for line in buf_reader.lines() {
        let line = line.unwrap();

        let mut chunks = line.split_whitespace();
        match chunks.next().unwrap() {
            "forward" => { pos += chunks.next().unwrap().parse::<isize>().unwrap(); }
            "down" => { depth += chunks.next().unwrap().parse::<isize>().unwrap(); }
            "up" => { depth -= chunks.next().unwrap().parse::<isize>().unwrap(); }
            _ => {}
        }
    }

    println!("Depth: {}", depth);
    println!("Pos: {}", pos);
    println!("Product: {}", depth * pos);
}

fn part2(buf_reader: BufReader<File>) {
    let mut aim = 0;
    let mut depth = 0;
    let mut pos = 0;

    for line in buf_reader.lines() {
        let line = line.unwrap();

        let mut chunks = line.split_whitespace();
        match chunks.next().unwrap() {
            "forward" => { 
                let val = chunks.next().unwrap().parse::<isize>().unwrap();
                pos += val;
                depth += val * aim;
            }
            "down" => { aim += chunks.next().unwrap().parse::<isize>().unwrap(); }
            "up" => { aim -= chunks.next().unwrap().parse::<isize>().unwrap(); }
            _ => {}
        }
    }

    println!("Depth: {}", depth);
    println!("Pos: {}", pos);
    println!("Product: {}", depth * pos);
}

fn main() {
    // Read file line by line
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);

    //part1(buf_reader);
    part2(buf_reader);
}
