use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp;
#[macro_use] extern crate itertools;


fn main() {
    // Read file line by line
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);

    let lines: Vec<Vec<usize>> = buf_reader.lines().map(|x| {
        x.unwrap().split(" -> ").map(|pair| { pair.split(',').map(|n| { n.parse().unwrap() }).collect::<Vec<usize>>() }).flatten().collect()
    }).collect();

    let mut max_x = 0;
    let mut max_y = 0;
    for line in &lines {
        if let [x1, y1, x2, y2] = line[0..4] {
            if x1 > max_x { max_x = x1 }
            if x2 > max_x { max_x = x2 }
            if y1 > max_y { max_y = y1 }
            if y2 > max_y { max_y = y2 }
        }
    }

    let mut map = vec![vec![0; max_x+1]; max_y+1];

    for line in &lines {
        if let [x1, y1, x2, y2] = line[0..4] {
            //println!("{}, {} -> {}, {}", x1, y1, x2, y2);
            if x1 == x2 {
                // Verti
                let start_y = cmp::min(y1, y2);
                let end_y = cmp::max(y1, y2);
                for i in start_y..=end_y {
                    map[i][x1] += 1;
                }
            } else if y1 == y2 {
                // Hori
                let start_x = cmp::min(x1, x2);
                let end_x = cmp::max(x1, x2);
                for i in start_x..=end_x {
                    map[y1][i] += 1;
                }
            } else {
                // If day1
                // continue;
                
                // Diag
                let start_x: usize;
                let end_x: usize;
                let start_y: usize;
                let end_y: usize;
                if x1 > x2 {
                    start_x = x2;
                    end_x = x1;
                    start_y = y2;
                    end_y = y1;
                } else {
                    start_x = x1;
                    end_x = x2;
                    start_y = y1;
                    end_y = y2;
                }
                // d_y_x is how much y changes from start_x to end_x
                let d_y_x = (end_y as isize - start_y as isize) / (end_x as isize - start_x as isize);

                for (i, x) in (start_x..=end_x).enumerate() {
                    let y = start_y as isize + (i as isize * d_y_x);
                    map[y as usize][x] += 1;
                }
            }
        }
    }

    // Print Map for debugging
    //for (y, x) in iproduct!(0..=max_y, 0..=max_x) {
    //    if map[y][x] == 0 {
    //        print!(".")
    //    } else {
    //        print!("{}", map[y][x])
    //    }
    //    if x == max_x {
    //        print!("\n")
    //    }
    //}

    let mut count = 0;
    for (y, x) in iproduct!(0..=max_y, 0..=max_x) {
        if map[y][x] >= 2 {
            count += 1;
        }
    }

    println!("Number of points > 2: {}", count);
}
