use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Read file line by line
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);

    let heights: Vec<Vec<u32>> = buf_reader
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let height = heights.len();
    let width = heights[0].len();

    let mut seen_count = vec![vec![0; width]; height];

    for y in 1..height - 1 {
        let mut left = heights[y][0];
        let mut right = heights[y][width - 1];

        seen_count[y][0] += 1;
        seen_count[y][width - 1] += 1;

        for x in 1..width {
            let l_c = heights[y][x];
            if l_c > left {
                seen_count[y][x] += 1;
                left = l_c;
            }

            let r_c = heights[y][width - (x + 1)];
            if r_c > right {
                seen_count[y][width - (x + 1)] += 1;
                right = r_c;
            }
        }
    }

    for x in 0..width {
        let mut top = heights[0][x];
        let mut bottom = heights[height - 1][x];

        seen_count[0][x] += 1;
        seen_count[height - 1][x] += 1;

        for y in 1..height {
            let t_c = heights[y][x];
            if t_c > top {
                seen_count[y][x] += 1;
                top = t_c;
            }

            let b_c = heights[height - (y + 1)][x];
            if b_c > bottom {
                seen_count[height - (y + 1)][x] += 1;
                bottom = b_c;
            }
        }
    }

    let total_visible: usize = seen_count
        .iter()
        .map(|row| {
            row.iter()
                .map(|v| if *v > 0 { 1 } else { 0 })
                .sum::<usize>()
        })
        .sum();

    println!("star1: {}", total_visible);

    let scores: Vec<Vec<u32>> = heights
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, h)| {
                    // Niave... walk to the first collision
                    // Improvement would be to make these iterators in each direction
                    let mut c_up = 0;
                    for up in 1..=y {
                        c_up += 1;
                        let t = heights[y - up][x];
                        if t >= *h {
                            break;
                        }
                    }

                    let mut c_down = 0;
                    for down in y + 1..height {
                        c_down += 1;
                        let t = heights[down][x];
                        if t >= *h {
                            break;
                        }
                    }

                    let mut c_left = 0;
                    for left in 1..=x {
                        c_left += 1;
                        let t = heights[y][x - left];
                        if t >= *h {
                            break;
                        }
                    }

                    let mut c_right = 0;
                    for right in x + 1..width {
                        c_right += 1;
                        let t = heights[y][right];
                        if t >= *h {
                            break;
                        }
                    }

                    let score = c_up * c_left * c_right * c_down;
                    return score;
                })
                .collect()
        })
        .collect();

    let best = scores.iter().flatten().max().unwrap();
    println!("star2: {}", best);
}
