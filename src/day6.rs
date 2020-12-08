use std::{collections::HashSet, fs};

pub fn run() {
    let input = fs::read_to_string("inputs/day6.txt").expect("Failed to read file");
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut count = 0;
    let mut cur_group = String::new();
    for line in input.lines() {
        if line.is_empty() {
            count += cur_group.chars().collect::<HashSet<char>>().len();
            cur_group.clear();
        } else {
            cur_group.push_str(line);
        }
    }
    count += cur_group.chars().collect::<HashSet<char>>().len();
    println!("{}", count);
}

fn part2(input: &str) {
    let mut count = 0;
    let mut cur_group: HashSet<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    for line in input.lines() {
        if line.is_empty() {
            count += cur_group.len();
            cur_group = "abcdefghijklmnopqrstuvwxyz".chars().collect();
        } else {
            cur_group = &cur_group & &line.chars().collect::<HashSet<char>>();
        }
    }
    count += cur_group.len();
    println!("{}", count);
}
