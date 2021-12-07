use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;


fn main() {
    // Read file line by line
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);

    let seed: Vec<usize> = buf_reader.lines().next().unwrap().unwrap().split(',').map(|n| { n.parse().unwrap() }).collect();
    let days_of_sim = 256;
    //let seed = vec!(1);

    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();

    println!("Input: {:?}", seed);
    let total: usize = seed.iter().map(|x| { how_many_fish(&mut cache, *x , days_of_sim) }).sum();
    println!("Total lanternfish: {}", total)
}

fn how_many_fish(cache: &mut HashMap<(usize, usize), usize>, til_breed: usize, days_left: usize) -> usize {
    match cache.get(&(til_breed, days_left)) {
        Some(result) => {
            *result
        },
        None => {
            let result = if til_breed >= days_left { 
                1
            } else {
                how_many_fish(cache, 6, days_left-(til_breed + 1)) + how_many_fish(cache, 8, days_left-(til_breed + 1))
            };

            cache.insert((til_breed, days_left), result);
            result
        }
    }
}
