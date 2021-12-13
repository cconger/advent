use std::fs::File;
use std::io::{BufRead, BufReader};

enum Fold {
    Horiz(usize),
    Verti(usize),
}

fn main() {
    // Read file line by line
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);

    
    let mut points = true;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut pts: Vec<(usize, usize)> = Vec::new();
    let mut folds: Vec<Fold> = Vec::new();
    for l in buf_reader.lines() {
        let l = l.unwrap();

        if l.len() == 0 {
            points = false;
            continue;
        }

        if points {
            let mut sp = l.split(",");
            let x = sp.next().unwrap().parse().unwrap();
            let y = sp.next().unwrap().parse().unwrap();
            if x > max_x { max_x = x; }
            if y > max_y { max_y = y; }

            pts.push((x,y));
        } else {
            let mut fold_split = l.split("=");
            let direction = fold_split.next().unwrap().chars().nth(11).unwrap();
            let value: usize = fold_split.next().unwrap().parse().unwrap();

            match direction {
                'x' => {
                    folds.push(Fold::Verti(value));
                },
                'y' => {
                    folds.push(Fold::Horiz(value));
                },
                _ => {
                    println!("Parse error!");
                    return;
                }
            }
        }
    }

    for f in folds {
        pts = pts.iter().map(|(x,y)| {
            match f {
                Fold::Horiz(h) => {
                    if *y > h {
                        let yp = h - (*y - h);
                        return (*x, yp);
                    }
                    max_y = h;
                    return (*x, *y);
                },
                Fold::Verti(v) => {
                    if *x > v {
                        let xp = v - (*x - v);
                        return (xp, *y);
                    }
                    max_x = v;
                    return (*x, *y);
                },
            }
        }).collect();

        // Part1 break
        // Sum all points
    }

    let mut map = vec![vec![0; max_x]; max_y];
    for (x,y) in pts {
        map[y][x] = 1;
    }

    for y in map {
        for x in y {
            if x > 0 {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}


