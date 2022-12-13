use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;

#[derive(Debug, Clone)]
enum Data {
    List(Vec<Data>),
    Int(u32),
}

fn parse(s: &str) -> Data {
    let mut stack: Vec<Vec<Data>> = vec![];

    let mut number: Option<u32> = None;

    for c in s.chars() {
        match c {
            '[' => {
                // create a new vec
                let new_v: Vec<Data> = vec![];
                stack.push(new_v)
            }
            ',' | ']' => {
                match number {
                    Some(n) => {
                        stack.last_mut().unwrap().push(Data::Int(n));
                        number = None;
                    }
                    None => {}
                }

                if c == ']' {
                    if stack.len() > 1 {
                        let done = stack.pop().unwrap();
                        stack.last_mut().unwrap().push(Data::List(done));
                    }
                }
                // pop vec stack
            }
            '0'..='9' => {
                // push integer digit
                number = match number {
                    None => Some(c.to_digit(10).unwrap()),
                    Some(n) => Some(n * 10 + c.to_digit(10).unwrap()),
                }
            }
            _ => {
                panic!("Unexpected input");
            }
        }
    }

    return Data::List(stack.pop().unwrap());
}

fn cmp(left: &Data, right: &Data) -> Ordering {
    match left {
        Data::List(l) => {
            match right {
                Data::List(r) => {
                    for (l, r) in zip(l.iter(), r.iter()) {
                        let v = cmp(l, r);

                        if !v.is_eq() {
                            return v;
                        }
                    }

                    if l.len() < r.len() {
                        return Ordering::Less;
                    }

                    if r.len() < l.len() {
                        return Ordering::Greater;
                    }

                    return Ordering::Equal;
                }
                Data::Int(r) => {
                    return cmp(left, &Data::List(vec![Data::Int(*r)]));
                }
            };
        }
        Data::Int(l) => {
            match right {
                Data::List(_) => {
                    return cmp(&Data::List(vec![Data::Int(*l)]), right);
                }
                Data::Int(r) => {
                    return l.cmp(r);
                }
            };
        }
    }
}

fn main() {
    // Read file line by line
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);

    let mut left: Data = Data::List(vec![]);
    let mut right: Data = Data::List(vec![]);

    let mut idxs = 0;
    let mut idx = 1;
    let mut c = 0;

    let sort_a = Data::List(vec![Data::List(vec![Data::Int(2)])]);
    let sort_b = Data::List(vec![Data::List(vec![Data::Int(6)])]);

    let mut all_packets: Vec<Data> = vec![sort_a.clone(), sort_b.clone()];
    for line in buf_reader.lines() {
        let line = line.unwrap();
        if line == "" {
            c = 0;
            idx += 1;
            continue;
        }

        if c == 0 {
            left = parse(&line);
            all_packets.push(left.clone());
            c += 1;
            continue;
        }

        if c == 1 {
            right = parse(&line);
            all_packets.push(right.clone());
        }

        let o = cmp(&left, &right);
        /*println!("===============");
        println!("IDX {} ORDERED {:?} ", idx, o);
        println!("left: {:?}", left);
        println!("right: {:?}", right);
        */

        if o.is_lt() {
            idxs += idx;
        }
    }

    println!("star1: {}", idxs);

    all_packets.sort_by(|a, b| cmp(&a, &b));
    let mut a = 0;
    let mut b = 0;
    for (i, p) in all_packets.iter().enumerate() {
        if cmp(&p, &sort_a).is_eq() {
            a = i + 1;
        }
        if cmp(&p, &sort_b).is_eq() {
            b = i + 1;
        }
        //println!("{:?}", p);
    }

    println!("star2: {}", a * b);
}
