use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone, Debug)]
struct Range {
    low: usize,
    high: usize,
}

fn parse_range(s: &str) -> Range {
    let nums: Vec<usize> = s.split('-').map(|x| x.parse().unwrap()).collect();

    return Range {
        low: nums[0],
        high: nums[1],
    };
}

fn contained_within(a: &Range, b: &Range) -> bool {
    if a.low <= b.low {
        if a.high >= b.high {
            return true;
        }
    }

    if b.low <= a.low {
        return b.high >= a.high;
    }

    return false;
}

fn overlapped(a: &Range, b: &Range) -> bool {
    if a.low <= b.low && a.high >= b.low {
        return true;
    }

    if b.low <= a.low && b.high >= a.low {
        return true;
    }

    return false;
}

fn main() {
    // Read file line by line
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);

    let mut contained = 0;
    let mut over = 0;
    for lin in buf_reader.lines() {
        let line = String::from(lin.unwrap());

        let mut i = line.split(',');
        let first_range = parse_range(i.next().unwrap());
        let second_range = parse_range(i.next().unwrap());

        if contained_within(&first_range, &second_range) {
            //println!("{:?} and {:?}", &first_range, &second_range);
            contained += 1;
        }

        if overlapped(&first_range, &second_range) {
            over += 1;
        }
    }

    println!("star1: {}", contained);
    println!("star2: {}", over);
}
