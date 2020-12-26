use std::{collections::HashSet, fs};

pub fn run(){
    let input = fs::read_to_string("inputs/day9.txt").expect("Failed to read file");
    let raw_data: Vec<u64> = input.lines().map(|line| line.parse::<u64>().unwrap()).collect();

    part1(&raw_data);
    part2(&raw_data);
}

fn part1(raw_data: &[u64]) {
    let xmas = XMAS::new(raw_data, 25);
    println!("{}", xmas.find_invalid_num().unwrap());
}

fn part2(raw_data: &[u64]) {
    let xmas = XMAS::new(raw_data, 25);
    println!("{}", xmas.find_weakness().unwrap());
}

struct XMAS {
    preamble: usize,
    data: Vec<u64>
}

impl XMAS {
    fn new(raw_data: &[u64], preamble: usize) -> Self {
        XMAS {
            preamble,
            data: Vec::from(raw_data)
        }
    }

    fn find_invalid_num(&self) -> Option<u64> {
        for i in self.preamble..self.data.len() {
            let mut sums = HashSet::new();
            for j in i - self.preamble..i {
                for k in i - self.preamble..i {
                    if j != k {
                        sums.insert(self.data[j] + self.data[k]);
                    }
                }
            }
            if !sums.contains(&self.data[i]) {
                return Some(self.data[i]);
            }
        }
        None
    }

    fn find_weakness(&self) -> Option<u64> {
        let mut step = 2;
        let invalid_num = self.find_invalid_num().unwrap();
        while step <= self.data.len() {
            for i in 0..=self.data.len() - step {
                let set = &self.data[i..i + step];
                if set.iter().sum::<u64>() == invalid_num {
                    return Some(set.iter().min().unwrap() + set.iter().max().unwrap());
                }
            }
            step += 1;
        }
        None
    }
}