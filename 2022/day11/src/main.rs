use std::fs::File;
use std::io::{BufRead, BufReader};

#[macro_use]
extern crate scan_fmt;

#[derive(Debug)]
enum Op {
    Mul(usize),
    Add(usize),
    MulSelf,
    AddSelf,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: Op,
    test_value: usize,
    true_target: usize,
    false_target: usize,
}

impl Monkey {
    fn process_items(&mut self, me: usize, targets: &mut [&mut Monkey], lcm: usize) -> usize {
        for i in &self.items {
            let mut i = match self.operation {
                Op::Mul(v) => i * v,
                Op::Add(v) => i + v,
                Op::MulSelf => i * i,
                Op::AddSelf => i + i,
            };

            i = i % lcm;

            if i % self.test_value == 0 {
                if self.true_target > me {
                    targets[self.true_target - 1].items.push(i);
                } else {
                    targets[self.true_target].items.push(i);
                }
            } else {
                if self.false_target > me {
                    targets[self.false_target - 1].items.push(i);
                } else {
                    targets[self.false_target].items.push(i);
                }
            }
        }

        let count = self.items.len();
        self.items = vec![];

        return count;
    }
}

fn main() {
    // Read file line by line
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);

    let mut lines = buf_reader.lines();

    let mut monkies: Vec<Monkey> = vec![];

    while let Some(line) = lines.next() {
        // Label doesn't matter...
        let line = line.unwrap();
        if line == "" {
            lines.next();
        }

        let starting_items = lines.next().unwrap().unwrap();

        let items: Vec<usize> = starting_items
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .map(|n| n.parse().unwrap())
            .collect();

        let op_val: Op;

        let operation = lines.next().unwrap().unwrap();

        let (op, val) =
            scan_fmt_some!(&operation, "  Operation: new = old {[*+]} {}", char, String);

        op_val = match val.unwrap().as_str() {
            "old" => match op.unwrap() {
                '+' => Op::AddSelf,
                '*' => Op::MulSelf,
                _ => panic!("unknown op type"),
            },
            v => {
                let v = v.parse().unwrap();

                match op.unwrap() {
                    '+' => Op::Add(v),
                    '*' => Op::Mul(v),
                    _ => panic!("unknown op type"),
                }
            }
        };

        let test_val: usize;
        let test = lines.next().unwrap().unwrap();
        if let Ok(target) = scan_fmt!(&test, "  Test: divisible by {d}", usize) {
            test_val = target;
        } else {
            panic!("Unable to parse test string");
        }

        let true_target: usize;
        let true_branch = lines.next().unwrap().unwrap();
        if let Ok(target) = scan_fmt!(&true_branch, "  If true: throw to monkey {d}", usize) {
            true_target = target;
        } else {
            panic!("Unable to parse true target string");
        }

        let false_target: usize;
        let false_branch = lines.next().unwrap().unwrap();
        if let Ok(target) = scan_fmt!(&false_branch, "  If false: throw to monkey {d}", usize) {
            false_target = target;
        } else {
            panic!("Unable to parse false target string");
        }

        monkies.push(Monkey {
            items,
            operation: op_val,
            test_value: test_val,
            true_target,
            false_target,
        });
    }

    // mutliply all prime divsors and use this as pressure outlet
    let lcm = monkies.iter().map(|m| m.test_value).product::<usize>();

    let rounds = 10000;

    let mut ops: Vec<usize> = vec![0; monkies.len()];
    for i in 0..rounds {
        println!("\n\nON ROUND {}", i);
        for m in &monkies {
            println!("{:?}", m);
        }

        for i in 0..monkies.len() {
            let (before, after) = monkies.split_at_mut(i);

            // Holy shit this is ugly
            if let Some((m, rest)) = after.split_first_mut() {
                let mut friends: Vec<&mut Monkey> =
                    before.iter_mut().chain(rest.iter_mut()).collect();
                ops[i] += m.process_items(i, &mut friends, lcm);
            }
        }
    }

    println!("\n\nFINAL");
    for m in monkies {
        println!("{:?}", m);
    }

    ops.sort();
    println!("star1: {}", ops[ops.len() - 1] * ops[ops.len() - 2]);
}
