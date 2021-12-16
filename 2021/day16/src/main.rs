use std::io::{self, BufRead};
use std::fmt;

fn main() -> io::Result<()> {
    println!("Reading input from stdin");

    // Read stdin
    let mut buffer = String::new();
    io::stdin().lock().read_line(&mut buffer)?;

    let input = buffer.chars().collect();

    let mut bit_iter = HexIter::new(input);

    let (_, p) = read_packet(&mut bit_iter);
    println!("Part 1 Version Sum: {}", version_sum(&p));
    println!("Part 2 Evaluated: {}", execute(&p));

    println!("{}", p);

    Ok(())
}

fn version_sum(p: &Packet) -> u64 {
    let mut acc = p.version as u64;

    for sub_p in &p.packets {
        acc += version_sum(sub_p);
    }

    return acc;
}

fn execute(p: &Packet) -> u64 {
    match p.id {
        0 => {
            // Sum
            p.packets.iter().map(|sub_p| execute(sub_p)).sum()
        },
        1 => {
            // Product
            p.packets.iter().map(|sub_p| execute(sub_p)).product()
        },
        2 => {
            // Minimum
            p.packets.iter().map(|sub_p| execute(sub_p)).min().unwrap()
        },
        3 => {
            // Maximum
            p.packets.iter().map(|sub_p| execute(sub_p)).max().unwrap()
        },
        4 => {
            p.literal
        },
        5 => {
            // Greater Than
            let first = execute(&p.packets[0]);
            let second = execute(&p.packets[1]);
            if first > second {
                1
            } else {
                0
            }
        },
        6 => {
            // Less than Than
            let first = execute(&p.packets[0]);
            let second = execute(&p.packets[1]);
            if first < second {
                1
            } else {
                0
            }
        },
        7 => {
            // Equal to
            let first = execute(&p.packets[0]);
            let second = execute(&p.packets[1]);
            if first == second {
                1
            } else {
                0
            }
        },
        _ => {
            panic!("Unexpected packet id {}", p.id);
        }
    }
}

fn read_packet(iter: &mut HexIter) -> (usize, Packet)  {
    let version = iter.get_next(3).unwrap() as u8;
    let id = iter.get_next(3).unwrap() as u8;
    let mut bits_eaten = 6;

    if id == 4 {
        let mut payload: u64 = 0;
        while let Some(p) = iter.get_next(5) {
            bits_eaten += 5;
            let v = p & 0b0000_1111;
            payload = (payload << 4) + v;
            if p & 0b0001_0000 == 0 {
                break;
            }
        }
        return (bits_eaten,
                Packet {
                    version,
                    id,
                    packets: vec![],
                    literal: payload,
                });
            
    }

    let size_flag = iter.get_next(1).unwrap();
    bits_eaten += 1;

    let mut packets = Vec::new();

    // size_flag 0 is read num of bytes
    if size_flag == 0 {
        let payload_size = iter.get_next(15).unwrap();
        bits_eaten += 15;

        let mut acc = 0;

        while (acc as u64) < payload_size {
            let (b, p) = read_packet(iter);
            bits_eaten += b;
            acc += b;
            packets.push(p);
        }
        if acc as u64 != payload_size {
            eprintln!("We parsed too few bits!");
        }
    }

    // size_flag 0 is num of packets
    if size_flag == 1 {
        let packet_count = iter.get_next(11).unwrap();
        bits_eaten += 11;

        for _ in 0..packet_count {
            let (b, p) = read_packet(iter);
            bits_eaten += b;
            packets.push(p);
        }
    }

    return (bits_eaten,
            Packet {
                version,
                id,
                packets,
                literal: 0
            });
}

struct Packet {
    version: u8, 
    id: u8,
    packets: Vec<Packet>,
    literal: u64,
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.id {
            0 => {
                // Sum
                write!(f, "(")?;
                for (i, sub_p) in self.packets.iter().enumerate() {
                    if i != 0 { write!(f, "+")?; }
                    { write!(f, "{}", sub_p)?; }
                }
                write!(f, ")")?;
            },
            1 => {
                // Product
                write!(f, "(")?;
                for (i, sub_p) in self.packets.iter().enumerate() {
                    if i != 0 { write!(f, "*")?; }
                    { write!(f, "{}", sub_p)?; }
                }
                write!(f, ")")?;
            },
            2 => {
                // Minimum
                write!(f, "min(")?;
                for (i, sub_p) in self.packets.iter().enumerate() {
                    if i != 0 { write!(f, ", ")?; }
                    { write!(f, "{}", sub_p)?; }
                }
                write!(f, ")")?;
            },
            3 => {
                // Maximum
                write!(f, "max(")?;
                for (i, sub_p) in self.packets.iter().enumerate() {
                    if i != 0 { write!(f, ", ")?; }
                    { write!(f, "{}", sub_p)?; }
                }
                write!(f, ")")?;
            },
            4 => {
                write!(f, "{}", self.literal)?;
            },
            5 => {
                // Greater Than
                write!(f, "({} > {})", self.packets[0], self.packets[1])?;
            },
            6 => {
                // Less than Than
                write!(f, "({} < {})", self.packets[0], self.packets[1])?;
            },
            7 => {
                // Equal to
                write!(f, "({} == {})", self.packets[0], self.packets[1])?;
            },
            _ => {
                write!(f, "XXX")?;
            }
        }
        return Ok(())
    }
}

// HexIter wraps a list of characters and operates as an iterator for the next bit
// Has a helper function for getting the next n bits as a single number.
struct HexIter {
    chars: Vec<char>,

    index: usize,
    current_char: u8,
    current_mask: u8,
}

impl HexIter {
    fn new(chars: Vec<char>) -> Self {
        Self {
            chars,
            index: 0,
            current_char: 0,
            current_mask: 0,
        }
    }

    // get_next returns the next `bits` # of bits. Yes its dumb that we flatten to bits then
    // recompose to a number here, but the mental purity is appreciated
    fn get_next(&mut self, bits: u8) -> Option<u64> {
        let mut v = 0;
        for i in 0..bits {
            match self.next() {
                Some(b) => {
                    v = (v << 1) + b as u64
                },
                None => {
                    if i == 0 { return None; }
                    return Some(v);
                }
            }
        }
        return Some(v);
    }
}

impl Iterator for HexIter {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if self.current_mask == 0 {
            if self.index >= self.chars.len() { return None; }

            let next_char = self.chars[self.index];
            self.index += 1;
            self.current_char = next_char.to_digit(16).unwrap() as u8;
            self.current_mask = 0b0000_1000;
        }

        let v = self.current_char & self.current_mask;
        self.current_mask = self.current_mask >> 1;
        if v > 0 {
            return Some(1);
        } else {
            return Some(0);
        }
    }
}
