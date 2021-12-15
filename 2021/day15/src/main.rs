use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::VecDeque;

fn main() {
    // Read file line by line
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);


    let map = buf_reader.lines().map(|line| { 
        let line = line.unwrap();
        line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect()
    }).collect::<Vec<Vec<u8>>>();

    part1(&map);

    part2(&map);
}

// Part1 returns the lowest risk path to the lower right position
fn part1(map: &Vec<Vec<u8>>) {
    let lowest_risk = min_score((0,0), (map[0].len() - 1, map.len() - 1), map);
    //let (path, lowest_risk) = dfs((0,0), 0, &vec![], map).unwrap();
    println!("Part1:");
    println!("Risk: {}", lowest_risk);
}

fn part2(map: &Vec<Vec<u8>>) {
    // Make it much bigger and then run same algo?

    let modifiers = [
        0, 1, 2, 3, 4,
        1, 2, 3, 4, 5,
        2, 3, 4, 5, 6,
        3, 4, 5, 6, 7,
        4, 5, 6, 7, 8
    ];

    let y_dim = map.len();
    let x_dim = map[0].len();
    let mut bmap = vec![vec![0; x_dim * 5]; y_dim * 5];
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            for my in 0..5 {
                for mx in 0..5 {
                    let mut v = map[y][x] + modifiers[my*5 + mx];
                    if v >= 10 { v -= 9 }
                    bmap[(y_dim * my) + y][(y_dim * mx) + x] = v;
                }
            }
        }
    }

    let lowest_risk = min_score((0,0), ((x_dim * 5) - 1, (y_dim * 5) - 1), &bmap);
    println!("Part2:");
    println!("Risk: {}", lowest_risk);
}

const DIRECTIONS: [(i64, i64);4] = [
    (-1, 0),
    (1, 0),
    (0, 1),
    (0, -1),
];

fn min_score(start: (usize, usize), end: (usize, usize), map: &Vec<Vec<u8>>) -> u64 {
    let y_max = map.len();
    let x_max = map[0].len();
    let mut min_scores = vec![vec![u64::MAX; map[0].len()];map.len()];

    let mut explorations: VecDeque<((usize, usize), u64)> = VecDeque::new();
    explorations.push_back((start, 0));

    while let Some((pos, risk)) = explorations.pop_front() {
        let (x, y) = pos;
        // If we found a worse way to get here... stop
        if risk >= min_scores[y][x] { continue; }
        min_scores[y][x] = risk;

        // Otherwise, add all neighbors to the search party.
        for d in DIRECTIONS {
            let (dx, dy) = d;
            let tx = x as i64 + dx;
            let ty = y as i64 + dy;
            if (tx >= 0) && ((tx as usize) < x_max) && (ty >= 0) && ((ty as usize) < y_max) {
                let next_risk = map[ty as usize][tx as usize];
                explorations.push_back(((tx as usize, ty as usize), risk + next_risk as u64));
            }
        };
    }

    return min_scores[end.1][end.0];
}
