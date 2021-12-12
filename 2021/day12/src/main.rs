use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

struct Node {
    neighbors: Vec<usize>,
    large: bool,
}

impl Node {
    fn new(name: String) -> Self {
        let large = name.to_uppercase() == name;
        Self {
            neighbors: Vec::new(),
            large,
        }
    }
}

fn get_or_insert(lookup: &mut HashMap<String, usize>, nodes: &mut Vec<Node>, key: String) -> usize {
    match lookup.get(&key) {
        Some(v) => *v,
        None => {
            nodes.push(Node::new(key.to_string()));
            let idx = nodes.len() - 1;
            lookup.insert(key, idx);
            idx
        }
    }
}

fn main() {
    // Read file line by line
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);

    // Graphs suck in rust due to memory opinions
    // So we're gonna do hash to index in vec (weak ref)
    let mut nodes: Vec<Node> = Vec::new();
    let mut lookup: HashMap<String, usize> = HashMap::new();

    let connections: Vec<String> = buf_reader.lines().map(|l| {
        return l.unwrap().split('-').map(|n| n.to_string()).collect::<Vec<String>>();
    }).flatten().collect();

    let mut start: usize = 0;
    let mut end: usize = 0;

    for i in (0..connections.len()).step_by(2) {
        let origin_idx = get_or_insert(&mut lookup, &mut nodes, connections[i].to_string());
        if connections[i] == "start" { start = origin_idx }
        if connections[i] == "end" { end = origin_idx }

        let dest_idx = get_or_insert(&mut lookup, &mut nodes, connections[i+1].to_string());
        if connections[i+1] == "start" { start = dest_idx }
        if connections[i+1] == "end" { end = dest_idx }

        let origin = &mut nodes[origin_idx];
        origin.neighbors.push(dest_idx);

        let dest = &mut nodes[dest_idx];
        dest.neighbors.push(origin_idx);
    }

    if start == end {
        println!("INIT FAILURE");
        return;
    }

    println!("Start: {}", start);
    println!("End: {}", end);
    let paths = walk_path(&nodes, &[], start, end, true);
    println!("Part 1 Total paths: {}", paths.len());

    let doublepaths = walk_path(&nodes, &[], start, end, false);
    println!("Part 2 Total paths: {}", doublepaths.len());
}

fn walk_path(nodes: &[Node], root: &[usize], node: usize, end: usize, did_second_visit: bool) -> Vec<Vec<usize>> {
    let mut new_root = root.to_vec();
    new_root.push(node);
    let mut paths: Vec<Vec<usize>> = Vec::new();

    for next in &nodes[node].neighbors {
        if *next == end {
            paths.push(new_root.to_vec());
            continue;
        }
        let mut second_visit = did_second_visit;
        if root.contains(&next) && !nodes[*next].large {
            if did_second_visit || (root.len() > 0 && *next == root[0]) { continue; }
            second_visit = true;
        }
        paths.extend(walk_path(nodes, &new_root, *next, end, second_visit));
    }
    paths
}
