use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {
    // Read file line by line
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);

    let mut lines = buf_reader.lines();

    let poly: Vec<char> = lines.next().unwrap().unwrap().chars().collect();

    lines.next();

    let rules: Vec<Rule> = lines.map(|l| {
        let l = l.unwrap();
        let mut s = l.split(" -> ");
        let pattern: Vec<char> = s.next().unwrap().chars().take(2).collect::<Vec<char>>();
        let insert: char = s.next().unwrap().chars().nth(0).unwrap();

        Rule {
            pattern: [pattern[0], pattern[1]],
            insert,
        }

    }).collect();

    pair_splitting("day1", poly.to_vec(), rules.to_vec(), 40);

    // Naive solutions below. Works for 10, but not 40 iters
    //day1(poly.to_vec(), rules.to_vec());
    //day2(poly.to_vec(), rules.to_vec());
}

fn pair_splitting(name: &str, poly: Vec<char>, rules: Vec<Rule>, iterations: usize) {

    let mut l_freq: HashMap<char, u64> = HashMap::default();

    let mut pairs: Vec<u64> = vec![0;rules.len()];
    for i in 0..poly.len() - 1 {
        *l_freq.entry(poly[i]).or_insert(0) += 1;
        let pair: [char;2] = [poly[i], poly[i+1]];
        match rules.iter().position(|r| r.pattern == pair) {
            Some(i) => {
                pairs[i] += 1
            }
            None => {}
        }
    }
    // Add the last character
    *l_freq.entry(poly[poly.len() - 1]).or_insert(0) += 1;

    for _ in 0..iterations {
        let mut next = vec![0; rules.len()];
        for (i, r) in rules.iter().enumerate() {
            let v = pairs[i];
            if v == 0 {
                continue
            }
            // AB => C
            // ACB
            // AC CB added (n times)
            // AB removed  (n times)
            //
            // C freq +n

            *l_freq.entry(r.insert).or_insert(0) += v;
            let left = [r.pattern[0], r.insert];
            let right = [r.insert, r.pattern[1]];

            match rules.iter().position(|r| r.pattern == left) {
                Some(x) => {
                    next[x] += v;
                }
                None => {}
            }
            match rules.iter().position(|r| r.pattern == right) {
                Some(x) => {
                    next[x] += v;
                }
                None => {}
            }
        }
        pairs = next;
    }

    let mut min_count = u64::MAX;
    let mut max_count = 0;
    for (_, v) in l_freq {
        if v < min_count { min_count = v; }
        if v > max_count { max_count = v; }
    }


    println!("{}", name);
    println!("Least Common Frequency: {}", min_count);
    println!("Most Common Frequency:  {}", max_count);
    println!("Delta: {}", max_count - min_count);
}

#[derive(Clone)]
struct Rule {
    pattern: [char;2],
    insert: char,
}

struct Polymer<'a> {
    previous: Vec<char>,
    rules: &'a Vec<Rule>,

    cursor: usize,
    inserted: bool,
}

impl Iterator for Polymer<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.previous.len() {
            return None;
        }

        if self.cursor == 0 {
            self.cursor += 1;
            return Some(self.previous[0]);
        }

        let pair = [self.previous[self.cursor - 1], self.previous[self.cursor]];
        for r in self.rules {
            if !self.inserted && pair == r.pattern {
                self.inserted = true;
                return Some(r.insert)
            }

        }
        self.inserted = false;
        self.cursor += 1;
        Some(self.previous[self.cursor - 1])
    }
}

fn day1(mut poly: Vec<char>, rules: Vec<Rule>) {
    for i in 0..10 {
        println!("Iteration: {}", i);
        let it = Polymer{
            previous: poly,
            rules: &rules,
            cursor: 0,
            inserted: false,
        };

        poly = it.collect();
    }

    let mut m: HashMap<char, u64> = HashMap::default();
    for v in &poly {
        *m.entry(*v).or_insert(0) += 1;
    }

    let mut min_count = u64::MAX;
    let mut max_count = 0;
    for (_, v) in m {
        if v < min_count { min_count = v; }
        if v > max_count { max_count = v; }
    }


    println!("Day 1");
    println!("Least Common Frequency: {}", min_count);
    println!("Most Common Frequency:  {}", max_count);
    println!("Delta: {}", max_count - min_count);
}


