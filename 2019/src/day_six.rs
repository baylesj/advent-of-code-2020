use std::collections::HashMap;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const INPUT_FILENAME: &'static str = "input/day_six.txt";

#[derive(Debug, Default)]
struct Node {
    parent: Option<String>,
    children: Vec<String>,
}

struct ParentChildPair(String, String);
fn parse_line(line: String) -> ParentChildPair {
    let v: Vec<&str> = line.split(")").collect();
    assert!(v.len() == 2);
    ParentChildPair(v[0].to_string(), v[1].to_string())
}

// TODO: hashmap lifetime is screwy.
fn parse_input<'a>() -> HashMap<String, Node> {
    let mut map: HashMap<String, Node> = HashMap::new();
    let file = File::open(INPUT_FILENAME).expect("Invalid filename");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let parent_child_pair = parse_line(line.expect("failed to read line"));

        let parent = map
            .entry(parent_child_pair.0.to_string())
            .or_insert(Node::default());
        parent.children.push(parent_child_pair.1.to_string());
        let child = map.entry(parent_child_pair.1).or_insert(Node::default());
        child.parent = Some(parent_child_pair.0.to_string());
    }

    map
}

fn part_one() -> i32 {
    let map = parse_input();
    const ROOT_NODE_NAME: &str = "COM";
    let mut current_level: Vec<&Node> = vec![map.get(&ROOT_NODE_NAME.to_string()).unwrap()];
    let mut current_level_depth: i32 = 0;
    let mut current_num_orbits = 0;
    while current_level.len() > 0 {
        let mut next_level: Vec<&Node> = Vec::new();
        for node in current_level {
            current_num_orbits += current_level_depth;
            for name in &node.children {
                next_level.push(&map[name]);
            }
        }
        current_level_depth += 1;
        current_level = next_level;
    }

    current_num_orbits
}

fn part_two() -> i32 {
    let map = parse_input();
    const SANTA_NODE_NAME: &str = "SAN";
    let santas_node = map.get(&SANTA_NODE_NAME.to_string());
    let mut santas_parents = HashMap::new();

    const YOUR_NODE_NAME: &str = "YOU";
    let your_node = map.get(&YOUR_NODE_NAME.to_string());

    let mut sa = santas_node.unwrap().parent.as_ref();
    let mut depth: i32 = 0;
    while let Some(a) = sa {
        santas_parents.insert(a, depth);
        depth += 1;
        sa = map[&a.to_string()].parent.as_ref();
    }

    let mut ya = your_node.unwrap().parent.as_ref();
    let mut your_depth: i32 = 0;
    while let Some(a) = ya {
        if santas_parents.contains_key(&a) {
            return santas_parents[&a] + your_depth;
        }
        your_depth += 1;
        ya = map[&a.to_string()].parent.as_ref();
    }
    -1
}

pub fn solve() -> String {
    format!("part one: {}, part two: {}", part_one(), part_two())
}
