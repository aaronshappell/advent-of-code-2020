use std::fs;
use regex::RegexSet;

pub fn run() {
    let input = fs::read_to_string("inputs/day4.txt").expect("Failed to read file");

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {let mut fields: Vec<&str> = Vec::new();
    let mut field_count = 0;
    let mut valid_count = 0;
    for line in input.lines() {
        if line.chars().count() == 0 {
            for field in fields.iter() {
                let pair: Vec<&str> = field.split(':').collect();
                match pair[0] {
                    "byr" | "iyr" | "eyr" | "hgt" | "hcl" | "ecl" | "pid" => field_count += 1,
                    "cid" => (),
                    _ => panic!("Invalid passport field")
                }
            }
            if field_count == 7 {
                valid_count += 1;
            }
            fields.clear();
            field_count = 0;
        } else {
            fields.extend(line.split(" "));
        }
    }
    println!("{}", valid_count);
}

fn part2(input: &str) {
    let re = RegexSet::new(&[
        r"byr:(19[2-9]\d|200[0-2])\b",
        r"iyr:(201\d|2020)\b",
        r"eyr:(202\d|2030)\b",
        r"hgt:((1[5-8]\d|19[0-3])cm|(59|6\d|7[0-6])in)\b",
        r"hcl:#[\da-f]{6}\b",
        r"ecl:(amb|blu|brn|gry|grn|hzl|oth)\b",
        r"pid:\d{9}\b"
    ]).unwrap();

    let mut field_count = 0;
    let mut valid_count = 0;
    for line in input.lines() {
        field_count += re.matches(line).iter().count();
        if line.chars().count() == 0 {
            if field_count == 7 {
                valid_count += 1;
            }
            field_count = 0;
        }
    }
    if field_count == 7 {
        valid_count += 1;
    }
    println!("{}", valid_count);
}