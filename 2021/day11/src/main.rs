use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::VecDeque;

extern crate ansi_term;
use ansi_term::Colour::Red;

fn main() {
    // Read file line by line
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);

    let octs: Vec<Vec<u32>> = buf_reader.lines().map(|l| {
        l.unwrap().chars().flat_map(|c| { c.to_digit(10) }).collect()
    }).collect();

    let mut part1 = octs.clone();
    let mut total_flashes = 0;
    for _ in 0..100 {
        total_flashes += simulate_step(&mut part1);
    }
    println!("Part1 Total Flashes After 100 Generations: {}", total_flashes);

    let mut part2 = octs.clone();
    let mut i = 0;
    loop {
        simulate_step(&mut part2);
        i += 1;
        let all_flash = part2.iter().flatten().fold(true, |acc, v| { acc && *v == 0});
        if all_flash {
            println!("Part2: All flashed after {} generations", i);
            print_octs(&part2);
            break;
        }
    }

}

fn simulate_step(octs: &mut Vec<Vec<u32>>) -> u32 {
    let mut flash_count = 0;
    let mut flashes = VecDeque::new();
    for y in 0..octs.len() {
        for x in 0..octs[y].len() {
            let v = octs[y][x] + 1;
            octs[y][x] = v;
            if v > 9 {
                flashes.push_back([x,y]);
            }
        }
    }

    while let Some([x,y]) = flashes.pop_front() {
        //println!("Processing flash ({}, {})", x, y);
        flash_count += 1;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 { continue; }
                let xp: i32 = x as i32 + dx;
                let yp: i32 = y as i32 + dy;
                if yp >= 0 && yp < octs.len() as i32 {
                    if xp >= 0 && xp < octs[yp as usize].len() as i32 {
                        //println!("Incrementing ({}, {})", xp, yp);
                        let y_i = yp as usize;
                        let x_i = xp as usize;
                        let v = octs[y_i][x_i as usize] + 1;
                        octs[y_i][x_i] = v;
                        if v == 10 {
                            flashes.push_back([x_i,y_i]);
                        }
                    }
                }
            }
        }
    }

    for y in 0..octs.len() {
        for x in 0..octs[y].len() {
            if octs[y][x] > 9 {
                octs[y][x] = 0;
            } 
        }
    }
    return flash_count;
}

fn print_octs(octs: &Vec<Vec<u32>>) {
    println!("===========");
    for y in octs {
        for x in y {
            let v = format!("{}", x);
            if *x == 0 {
                print!("{}", Red.bold().paint(&v));
            } else {
                print!("{}", v);
            }
        }
        print!("\n");
    }
    println!("===========\n");
}
