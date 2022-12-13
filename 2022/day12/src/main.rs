use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Visited {
    Not,
    Visited(usize),
}

#[derive(Debug)]
struct Spot {
    visited: Visited,
    x: usize,
    y: usize,
    height: u8,
}

fn main() {
    // Read file line by line
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);

    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);
    let mut map: Vec<Vec<Spot>> = buf_reader
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.unwrap()
                .chars()
                .enumerate()
                .map(|(x, c)| match c {
                    'S' => {
                        start = (x, y);
                        Spot {
                            visited: Visited::Visited(0),
                            x,
                            y,
                            height: 0,
                        }
                    }
                    'E' => {
                        end = (x, y);
                        Spot {
                            visited: Visited::Not,
                            x,
                            y,
                            height: 25,
                        }
                    }
                    'a'..='z' => Spot {
                        visited: Visited::Not,
                        x,
                        y,
                        height: c as u8 - 'a' as u8,
                    },
                    _ => {
                        panic!("unexpected input");
                    }
                })
                .collect()
        })
        .collect();

    // Breadth first search where we stop at visited locations
    // Enqueue only spaces that are traversable
    // Could improve by favoring climbing

    let mut queue = VecDeque::new();
    /* star1
    queue.push_back(start);


    let path = loop {
        let t = match queue.pop_front() {
            Some(t) => t,
            None => {
                break 0;
            }
        };

        let height;
        let depth;
        {
            let target = &map[t.1][t.0];
            if t == end {
                break match target.visited {
                    Visited::Not => 0,
                    Visited::Visited(v) => v,
                };
            }
            height = target.height;
            depth = match target.visited {
                Visited::Not => 0,
                Visited::Visited(v) => v,
            };
        }

        let mut dirs = Vec::with_capacity(4);

        if t.0 > 0 {
            // can go left
            dirs.push((t.0 - 1, t.1));
        }

        if t.0 < map[0].len() - 1 {
            // can go right
            dirs.push((t.0 + 1, t.1));
        }

        if t.1 > 0 {
            // can go up
            dirs.push((t.0, t.1 - 1));
        }

        if t.1 < map.len() - 1 {
            // can go down
            dirs.push((t.0, t.1 + 1));
        }

        for c in dirs {
            let n = &mut map[c.1][c.0];

            match n.visited {
                Visited::Visited(_) => {
                    continue;
                }
                _ => {}
            }

            if n.height <= height + 1 {
                n.visited = Visited::Visited(depth + 1);
                queue.push_back((n.x, n.y));
            }
        }
    };

    println!("star1: {}", path);
    */
    queue.push_back(end);

    let path = loop {
        let t = match queue.pop_front() {
            Some(t) => t,
            None => {
                break 0;
            }
        };

        let height;
        let depth;
        {
            let target = &map[t.1][t.0];
            if target.height == 0 {
                break match target.visited {
                    Visited::Not => 0,
                    Visited::Visited(v) => v,
                };
            }
            height = target.height;
            depth = match target.visited {
                Visited::Not => 0,
                Visited::Visited(v) => v,
            };
        }

        let mut dirs = Vec::with_capacity(4);

        if t.0 > 0 {
            // can go left
            dirs.push((t.0 - 1, t.1));
        }

        if t.0 < map[0].len() - 1 {
            // can go right
            dirs.push((t.0 + 1, t.1));
        }

        if t.1 > 0 {
            // can go up
            dirs.push((t.0, t.1 - 1));
        }

        if t.1 < map.len() - 1 {
            // can go down
            dirs.push((t.0, t.1 + 1));
        }

        for c in dirs {
            let n = &mut map[c.1][c.0];

            match n.visited {
                Visited::Visited(_) => {
                    continue;
                }
                _ => {}
            }

            if n.height >= height - 1 {
                n.visited = Visited::Visited(depth + 1);
                queue.push_back((n.x, n.y));
            }
        }
    };

    println!("star2: {}", path);
}
