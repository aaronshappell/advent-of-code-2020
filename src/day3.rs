use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/day3.txt").expect("Failed to read file");
    let map_width = input.lines().next().unwrap().chars().count();
    let map_height = input.lines().count();
    let mut map: Vec<char> = Vec::new();
    for line in input.lines() {
        map.extend(line.chars());
    }
    part1(&map, map_width, map_height);
    part2(&map, map_width, map_height);
}

fn count_trees(map: &[char], map_width: usize, map_height: usize, dx: usize, dy: usize) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut tree_count = 0;
    while y < map_height {
        if map[map_width * y + x % map_width] == '#' {
            tree_count += 1;
        }
        x += dx;
        y += dy;
    }
    tree_count
}

fn part1(map: &[char], map_width: usize, map_height: usize) {
    println!("{}", count_trees(map, map_width, map_height, 3, 1));
}

fn part2(map: &[char], map_width: usize, map_height: usize) {
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut product = 1;
    for slope in slopes.iter() {
        product *= count_trees(map, map_width, map_height, slope.0, slope.1);
    }
    println!("{}", product);
}