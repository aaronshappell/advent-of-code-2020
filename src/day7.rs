use std::{collections::HashMap, fs};

pub fn run() {
    let input = fs::read_to_string("inputs/day6.txt").expect("Failed to read file");
    let mut rules: HashMap<&str, Vec<Bag>> = HashMap::new();
    for line in input.lines() {
        // TODO regex line into a Bags and add to rules map
    }
    part1(&rules);
}

fn part1(rules: &HashMap<&str, Vec<Bag>>) {
    for bags in rules.values() {
        for bag in bags {
        }
    }
}

fn count_parents(bag: &Bag) -> u32 {
    
}

struct Bag {
    name: String,
    count: u32
}

impl Bag {
    fn new(name: &str, count: u32) -> Self {
        Bag {
            name: String::from(name),
            count
        }
    }
}