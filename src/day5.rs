use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/day5.txt").expect("Failed to read file");

    part1(&input);
}

fn part1(input: &str) {
    let mut max_seat_id = 0;
    for line in input.lines() {
        let row = find_index(&line[..7], 0, 128);
        let col = find_index(&line[7..], 0, 8);
        let seat_id = row * 8 + col;
        if seat_id > max_seat_id {
            max_seat_id = seat_id;
        }
    }
    println!("{}", max_seat_id);
}

fn find_index(pattern: &str, cur_pos: u32, size: u32) -> u32 {
    let next_pos = match pattern.chars().nth(0).unwrap() {
        'F' | 'L' => cur_pos,
        'B' | 'R' => cur_pos + size / 2,
        _ => panic!("Invalid char")
    };
    if size > 2 {
        return find_index(&pattern[1..], next_pos, size / 2);
    } else {
        return next_pos;
    }
}