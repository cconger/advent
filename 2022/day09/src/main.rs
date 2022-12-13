use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[macro_use]
extern crate scan_fmt;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

fn update_head(m: &Move, head: &mut Point) {
    match m {
        Move::Up => {
            head.y -= 1;
        }
        Move::Down => {
            head.y += 1;
        }
        Move::Left => {
            head.x -= 1;
        }
        Move::Right => {
            head.x += 1;
        }
    };
}

fn update_follow(head: &mut Point, follow: &mut Point) {
    // Right and down are positive
    let x_diff = head.x - follow.x;
    let y_diff = head.y - follow.y;

    if x_diff > 1 {
        // Rigth
        follow.x = head.x - 1;
        if !(y_diff > 1 || y_diff < -1) {
            follow.y = head.y;
        }
    }
    if x_diff < -1 {
        // Left
        follow.x = head.x + 1;
        if !(y_diff > 1 || y_diff < -1) {
            follow.y = head.y;
        }
    }
    if y_diff > 1 {
        // Down
        if !(x_diff > 1 || x_diff < -1) {
            follow.x = head.x;
        }
        follow.y = head.y - 1;
    }
    if y_diff < -1 {
        // Up
        if !(x_diff > 1 || x_diff < -1) {
            follow.x = head.x;
        }
        follow.y = head.y + 1;
    }
}

fn main() {
    // Read file line by line
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);

    let mut points: Vec<Point> = (0..10).map(|_| Point { x: 0, y: 0 }).collect();

    let mut set: HashSet<Point> = HashSet::new();

    for line in buf_reader.lines() {
        let line = line.unwrap();

        let (c, v) = scan_fmt_some!(&line, "{} {d}", char, usize);
        let v = v.unwrap();

        let m = match c.unwrap() {
            'U' => Move::Up,
            'D' => Move::Down,
            'L' => Move::Left,
            'R' => Move::Right,
            _ => panic!("unexpected input"),
        };

        for _ in 0..v {
            update_head(&m, points.get_mut(0).unwrap());
            for idx in 1..points.len() {
                let (head, rest) = points.split_at_mut(idx);
                update_follow(&mut head[head.len() - 1], &mut rest[0]);
            }

            set.insert(points.last().unwrap().clone());
        }
    }

    println!("star1: {}", set.len());
}

fn print_pnts(points: &[Point]) {
    let mut x_min = 0;
    let mut x_max = 1;
    let mut y_min = 0;
    let mut y_max = 1;

    for p in points {
        if p.x >= x_max {
            x_max = p.x + 1;
        }
        if p.x < x_min {
            x_min = p.x;
        }
        if p.y >= y_max {
            y_max = p.y + 1;
        }
        if p.y < y_min {
            y_min = p.y;
        }
    }
    let y_range = (y_max - y_min) as usize;
    let x_range = (x_max - x_min) as usize;

    let mut m: Vec<Vec<char>> = vec![vec!['.'; x_range]; y_range];
    for (i, p) in points.iter().enumerate().rev() {
        m[(p.y - y_min) as usize][(p.x - x_min) as usize] = char::from_digit(i as u32, 10).unwrap();
    }

    println!("=======");
    for r in m {
        for c in r {
            print!("{}", c);
        }
        print!("\n");
    }
}
