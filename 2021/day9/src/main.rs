use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::VecDeque;

fn main() {
    // Read file line by line
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);

    let map: Vec<Vec<u32>> = buf_reader.lines().map(|l| {
        l.unwrap().chars().flat_map(|c| { c.to_digit(10) }).collect()
    }).collect();

    let mut risk_sum = 0;
    let mut seeds: Vec<[usize;2]> = Vec::new();
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let v = map[y][x];
            let mut low = true;
            if x > 0 {
                low = low && (map[y][x-1] > v)
            }
            if x < map[y].len() - 1 {
                low = low && (map[y][x+1] > v)
            }
            if y > 0 {
                low = low && (map[y-1][x] > v)
            }
            if y < map.len() - 1 {
                low = low && (map[y+1][x] > v)
            }

            if low {
                //println!("Low found ({}, {}) = {}", x, y, v);
                seeds.push([x,y]);
                risk_sum += v + 1;
            }
        }
    }

    println!("Part1: {}", risk_sum);

    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    // Bredth first search from seeds
    let mut sizes: Vec<usize> = Vec::new();
    for [xo,yo] in seeds {
        let mut size = 1;
        let mut q = VecDeque::new();
        q.push_back([xo,yo]);

        while let Some([x,y]) = q.pop_front() {
            let v = map[y][x];
            if x > 0 {
                size += check_neighbor([x-1, y], v, &map, &mut visited, &mut q);
            }
            if y > 0 {
                size += check_neighbor([x, y-1], v, &map, &mut visited, &mut q);
            }
            size += check_neighbor([x+1, y], v, &map, &mut visited, &mut q);
            size += check_neighbor([x, y+1], v, &map, &mut visited, &mut q);
        }
        sizes.push(size);
        //println!("For low ({}, {}) Size: {}", xo,yo, size);
    }
    sizes.sort();
    let part2_sum: usize = sizes.iter().rev().take(3).product();
    println!("Part2 Product of sizes: {}", part2_sum);
}

fn check_neighbor([x,y]: [usize; 2], comp: u32, map: &Vec<Vec<u32>>, visited: &mut Vec<Vec<bool>>, q: &mut VecDeque<[usize;2]>) -> usize {
    if y >= map.len() { return 0 }
    if x >= map[y].len() { return 0}
    if visited[y][x] { return 0 }
    if map[y][x] <= comp { return 0 }
    if map[y][x] == 9 { return 0 }
    visited[y][x] = true;
    q.push_back([x,y]);
    return 1;
}
