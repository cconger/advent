use std::fs::File;
use std::io::{BufRead, BufReader};

// Length Mapping
// 2 on => 1
// 3 on => 7
// 4 on => 4
// 5 on => 2,3,5
// 6 on => 6,9,0
// 7 on => 8


fn main() {
    test();
    // Read file line by line
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);

    let mut sum = 0;
    for line in buf_reader.lines() {
        let line = line.unwrap();
        let mut lookup = [0; 10];
        if let [obs, display] = line.split(" | ").collect::<Vec<&str>>()[..] {
            let dis_enc: Vec<u8> = display.split_whitespace().map(|s| { encode(s) }).collect();

            for v in display.split_whitespace() {
                solve(v, true, &mut lookup);
            }

            for v in obs.split_whitespace() {
                solve(v, true, &mut lookup);
            }

            for v in obs.split_whitespace() {
                let n = solve(v, false, &mut lookup);
                if n > 9 {
                    println!("Could not resolve {}\nLookup Table: {:?}", v, lookup);
                    return;
                }

                let solvable = dis_enc.iter().fold(true, |acc, v| {
                    match lookup.iter().position(|t| t == v) {
                        Some(_) => acc && true,
                        None => acc && false,
                    }
                });
                if solvable {
                    let val = dis_enc.iter().fold(0, |acc, v| {
                        match lookup.iter().position(|t| t == v) {
                            Some(i) => acc * 10 + i,
                            None => 0,
                        }
                    });
                    //println!("Value: {}", val);
                    sum += val;
                    break;
                }
            }
        } else {
            println!("Failed to parse line");
            return
        }
    }
    println!("Sum: {}", sum);
}

fn decode_unique(input: &str) -> Result<i32, ()> {
    match input.len() {
        2 => Ok(1),
        3 => Ok(7),
        4 => Ok(4),
        7 => Ok(8),
        _ => Err(()),
    }
}

fn bits_in_u8(v: u8) -> u8 {
    let mut count = 0;
    for i in 0..7  {
        if (1 << i) & v > 0 { count+= 1; } 
    }
    count
}

// Encoding
//  0000
// 1    2
// 1    2
//  3333
// 4    5
// 4    5
//  6666
fn encode(a: &str) -> u8 {
    let mut v = 0;
    for c in a.chars() {
        match c {
            'a' => {v += 1 << 0;},
            'b' => {v += 1 << 1;},
            'c' => {v += 1 << 2;},
            'd' => {v += 1 << 3;},
            'e' => {v += 1 << 4;},
            'f' => {v += 1 << 5;},
            'g' => {v += 1 << 6;},
            _ => {},
        }
    }
    return v;
}

// This was manually done which was dumb...
fn test() {
    // dist map
    // Ambiguous nums
    // 2 from 1 = 5
    assert!(bits_in_u8(encode("gcdfa") ^ encode("ab")) == 5);
    // 2 from 7 = 4
    assert!(bits_in_u8(encode("gcdfa") ^ encode("dab")) == 4);
    // 2 from 4 = 5
    assert!(bits_in_u8(encode("gcdfa") ^ encode("eafb")) == 5);
    //
    // 3 from 1 = 3
    assert!(bits_in_u8(encode("fbcad") ^ encode("ab")) == 3);
    // 3 from 7 = 2
    assert!(bits_in_u8(encode("fbcad") ^ encode("dab")) == 2);
    // 3 from 4 = 3
    assert!(bits_in_u8(encode("fbcad") ^ encode("eafb")) == 3);
    //
    // 5 from 1 = 5
    assert!(bits_in_u8(encode("cdfbe") ^ encode("ab")) == 5);
    // 5 from 7 = 4
    assert!(bits_in_u8(encode("cdfbe") ^ encode("dab")) == 4);
    // 5 from 4 = 3
    assert!(bits_in_u8(encode("cdfbe") ^ encode("eafb")) == 3);
    //
    //
    // 
    // 6 from 1 = 6
    assert!(bits_in_u8(encode("cdfgeb") ^ encode("ab")) == 6);
    // 6 from 7 = 5
    assert!(bits_in_u8(encode("cdfgeb") ^ encode("dab")) == 5);
    // 6 from 4 = 3
    assert!(bits_in_u8(encode("cdfgeb") ^ encode("eafb")) == 4);
    //
    // 9 from 1 = 4
    assert!(bits_in_u8(encode("cefabd") ^ encode("ab")) == 4);
    // 9 from 7 = 3
    assert!(bits_in_u8(encode("cefabd") ^ encode("dab")) == 3);
    // 9 from 4 = 2
    assert!(bits_in_u8(encode("cefabd") ^ encode("eafb")) == 2);

    // 0 from 1 = 4
    assert!(bits_in_u8(encode("cagedb") ^ encode("ab")) == 4);
    // 0 from 7 = 3
    assert!(bits_in_u8(encode("cagedb") ^ encode("dab")) == 3);
    // 0 from 4 = 4
    assert!(bits_in_u8(encode("cagedb") ^ encode("eafb")) == 4);
}

fn solve(input: &str, pre: bool, solved: &mut [u8]) -> usize {
    let input_encoded = encode(input);
    match input.len() {
        2 => {
            solved[1] = input_encoded;
            return 1;
        },
        3 => {
            solved[7] = input_encoded;
            return 7;
        },
        4 => {
            solved[4] = input_encoded;
            return 4;
        },
        7 => {
            solved[8] = input_encoded;
            return 8;
        }
        5 => {
            if pre { return 11 }
            for c in [2,3,5] {
                if input_encoded ^ solved[c] == 0 {
                    return c;
                }
            }

            if bits_in_u8(input_encoded ^ solved[4]) == 3 {
                if bits_in_u8(input_encoded & solved[1]) == 2 {
                    solved[3] = input_encoded;
                    return 3;
                } else {
                    solved[5] = input_encoded;
                    return 5;
                }
            } else {
                solved[2] = input_encoded;
                return 2;
            }
        }
        6 => {
            if pre { return 11 }
            for c in [6,9,0] {
                if input_encoded ^ solved[c] == 0 {
                    return c;
                }
            }

            match bits_in_u8(input_encoded ^ solved[4]) {
                2 => {
                    solved[9] = input_encoded;
                    return 9;
                },
                4 => {
                    if bits_in_u8(input_encoded & solved[1]) == 2 {
                        solved[0] = input_encoded;
                        return 0;
                    } else {
                        solved[6] = input_encoded;
                        return 6;
                    }
                }
                _ => {
                    println!("Unexpected dist found");
                    return 11;
                }
            }
        }
        _ => {
            println!("Unexpected length found");
            return 11;
        }
    }
}
