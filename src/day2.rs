use std::fs;

pub fn run() {
    let input_file = fs::read_to_string("inputs/day2.txt").expect("Failed to read file");
    
    part1(&input_file);
    //part2(&data);
}

fn part1(data: &str) {
    let mut valid_count = 0;
    for line in data.lines() {
        let mut split_line: Vec<&str> = line.split(['-', ':', ' '].as_ref()).collect();
        split_line.remove(3);
        let range = split_line[0].parse::<i32>().unwrap()..split_line[1].parse::<i32>().unwrap() + 1;
        let letter = split_line[2].parse::<char>().unwrap();
        let password = split_line[3];

        if range.contains(&(password.chars().filter(|&c| c == letter).count() as i32)) {
            valid_count += 1;
        }
    }
    println!("{}", valid_count);
}