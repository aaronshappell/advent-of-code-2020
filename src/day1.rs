use std::fs;

pub fn run() {
    let input_file = fs::read_to_string("inputs/day1.txt").expect("Failed to read file");
    let mut data: Vec<i32> = Vec::new();
    for line in input_file.lines() {
        data.push(line.parse::<i32>().expect("Failed to parse i32"));
    }

    part1(&data);
    part2(&data);
}

fn part1(data: &[i32]) {
    'outer: for i in data {
        for j in data {
            if i + j == 2020 {
                println!("{} * {} = {}", i, j, i * j);
                break 'outer;
            }
        }
    }
}

fn part2(data: &[i32]) {
    'outer: for i in data {
        for j in data {
            for k in data {
                if i + j + k == 2020 {
                    println!("{} * {} * {} = {}", i, j, k, i * j * k);
                    break 'outer;
                }
            }
        }
    }
}