use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1(buf_reader: BufReader<File>) {
    let mut total = 0;
    let mut counts = [0; 12];
    for line in buf_reader.lines() {
        let line = line.unwrap();
        let int = isize::from_str_radix(&line, 2).unwrap();
        total += 1;
        for i in 0..12 {
            let mask = 1 << i;
            if int & mask > 0 {
                counts[i as usize] += 1;
            }
        }
    }

    println!("counts {:?}", counts);
    println!("total {}", total);

    let mut gamma: usize = 0;
    let mut epsilon: usize = 0;
    for (i, c) in counts.iter().enumerate() {
        println!("count in pos {}: {}", i, *c);
        if *c > total / 2 {
            gamma += 1 << i;
        } else {
            epsilon += 1 << i;
        }
    }

    println!("gamma: {}", gamma);
    println!("epsilon: {}", epsilon);
    println!("power: {}", gamma * epsilon);
}

fn part2(buf_reader: BufReader<File>) {
    let mut bits: usize = 0;
    let vals: Vec<isize> = buf_reader.lines().map(|x| {
        let line = x.unwrap();
        bits = line.len();
        isize::from_str_radix(&line, 2).unwrap()
    }).collect();

    let (on, off) = bit_split(vals, bits - 1);
    let mut oxygen: Vec<isize>;
    let mut co2: Vec<isize>;
    if on.len() > off.len() {
        oxygen = on;
        co2 = off;
    } else {
        oxygen = off;
        co2 = on;
    }

    for i in 2..bits + 1 {
        if oxygen.len() > 1 {
            let (on, off) = bit_split(oxygen, bits - i);
            if on.len() >= off.len() {
                oxygen = on
            } else {
                oxygen = off
            }
        }

        if co2.len() > 1 {
            let (on, off) = bit_split(co2, bits - i);
            if on.len() >= off.len() {
                co2 = off
            } else {
                co2 = on
            }
        }

        //println!("oxygen:");
        //oxygen.iter().for_each(|x| println!("{:012b}", x));
        //println!("co2:");
        //co2.iter().for_each(|x| println!("{:012b}", x));
        //println!("------");
    }

    println!("oxygen: {:?}", oxygen);
    println!("co2: {:?}", co2);

    println!("lifesupport: {}", oxygen[0] * co2[0]);
}

fn bit_split(candidates: Vec<isize>, position: usize) -> (Vec<isize>, Vec<isize>) {
    let mut on_vec = Vec::new();
    let mut off_vec = Vec::new();
    for c in candidates.iter() {
        if (c >> position) & 1 > 0 {
            on_vec.push(*c);
        } else {
            off_vec.push(*c);
        }
    }

    return (on_vec, off_vec);
}

fn main() {
    // Read file line by line
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);

    //part1(buf_reader);
    part2(buf_reader);
}
