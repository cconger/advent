use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Read file line by line
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);

    let seed: Vec<usize> = buf_reader.lines().next().unwrap().unwrap().split(',').map(|n| { n.parse().unwrap() }).collect();

    // Math trickery
    let mut nums = seed.clone();
    nums.sort();
    let piv1 = nums[nums.len()/2]; // median minimizes delta
    let piv2: usize = nums.iter().sum::<usize>() / nums.len(); // mean minimizes squared cost
    println!("Part1 Fuel Cost to Median({}): {}", piv1, costlinear(piv1, &nums));
    println!("WARNING doesn't work for all inputs:");
    println!("Part2 Fuel Cost to Average({}): {}", piv2, cost(piv2, &nums));


    // Was thinking about doing a binary search, but this was like... small
    // Do exhaustive search instead within the min and max
    let min = seed.iter().min().unwrap();
    let max = seed.iter().max().unwrap();

    let mut p = *min;
    let mut p_2 = *min;
    let mut c = costlinear(*min, &seed);
    let mut c_2 = cost(*min, &seed);
    for i in *min+1..=*max {
        let nc = cost(i, &seed);
        if nc < c_2 {
            c_2 = nc;
            p_2 = i;
        }
        let nc = costlinear(i, &seed);
        if nc < c {
            c = nc;
            p = i;
        }
    }


    println!("Part 1 best pivot {} with Fuel Cost: {}", p, c);
    println!("Part 2 best pivot {} with Fuel Cost: {}", p_2, c_2);
}

fn costlinear(pivot:usize, pos: &Vec<usize>) -> usize {
    pos.iter().fold(0, |acc, v| {
        if pivot > *v {
            acc + (pivot - v)
        } else {
            acc + (v - pivot)
        }
    })
}

fn cost(pivot: usize, pos: &Vec<usize>) -> usize {
    pos.iter().fold(0, |acc, v| {
        let offset: usize;
        if pivot > *v {
            offset = pivot - v;
        } else {
            offset = v - pivot;
        }
        acc + ((offset.pow(2) + offset) / 2)
    })
}
