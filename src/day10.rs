use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/day10_med.txt").expect("Failed to read file");

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut adapters: Vec<u32> = input.lines().map(|line| line.parse::<u32>().unwrap()).collect();
    adapters.sort();
    adapters.insert(0, 0);
    adapters.push(adapters.last().unwrap() + 3);
    let mut one_diff = 0;
    let mut three_diff = 0;
    for i in 1..adapters.len() {
        let diff = adapters[i] - adapters[i - 1];
        match diff {
            1 => one_diff += 1,
            3 => three_diff += 1,
            _ => ()
        }
    }
    println!("{:?}", adapters);
    println!("one:{}, three:{}", one_diff, three_diff);
    println!("{}", one_diff * three_diff);
}

fn part2(input: &str) {

}