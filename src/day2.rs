use std::fs;

struct PasswordEntry {
    lower: i32,
    upper: i32,
    letter: char,
    password: Vec<char>
}

pub fn run() {
    let input = fs::read_to_string("inputs/day2.txt").expect("Failed to read file");

    let mut password_db: Vec<PasswordEntry> = Vec::new();
    for line in input.lines() {
        let mut split_line: Vec<&str> = line.split(['-', ':', ' '].as_ref()).collect();
        split_line.remove(3);
        password_db.push(
            PasswordEntry {
                lower: split_line[0].parse::<i32>().unwrap(),
                upper: split_line[1].parse::<i32>().unwrap(),
                letter: split_line[2].parse::<char>().unwrap(),
                password: split_line[3].chars().collect()
            }
        );
    }
    
    part1(&password_db);
    part2(&password_db);
}

fn part1(entries: &[PasswordEntry]) {
    let mut valid_count = 0;
    for entry in entries {
        let letter_count = entry.password.iter().filter(|&&c| c == entry.letter).count() as i32;
        if letter_count >= entry.lower && letter_count <= entry.upper {
            valid_count += 1;
        }
    }
    println!("{}", valid_count);
}

fn part2(entries: &[PasswordEntry]) {
    let mut valid_count = 0;
    for entry in entries {
        let mut letter_count = 0;
        if entry.password[(entry.lower - 1) as usize] == entry.letter {
            letter_count += 1;
        }
        if entry.password[(entry.upper - 1) as usize] == entry.letter {
            letter_count += 1;
        }
        if letter_count == 1 {
            valid_count += 1;
        }
    }
    println!("{}", valid_count);
}