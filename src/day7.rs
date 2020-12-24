use std::fs;
use regex::Regex;

pub fn run() {
    let input = fs::read_to_string("inputs/day7.txt").expect("Failed to read file");
    let mut rules: Vec<Rule> = Vec::new();
    let bag_re = Regex::new(r"^(?P<name>.*?) bags").unwrap();
    let sub_bag_re = Regex::new(r" (?P<count>\d) (?P<name>.*?) bags?[,.]").unwrap();

    for line in input.lines() {
        let mut bag = Bag::new(&bag_re.captures(line).unwrap()["name"], 1);
        let mut sub_bags: Vec<Bag> = Vec::new();
        for cap in sub_bag_re.captures_iter(line) {
            sub_bags.push(Bag::new(&cap["name"], cap["count"].parse().unwrap()));
            if &cap["name"] == "shiny gold" {
                bag.contains_gold = true;
            }
        }
        rules.push(Rule { bag, sub_bags });
    }

    part1(&mut rules);
    part2(&rules);
}

fn part1(rules: &mut [Rule]) {
    Rule::update_parents(rules, "shiny gold");
    println!("{}", rules.iter().fold(0, |acc, r| if r.bag.contains_gold { acc + 1 } else { acc }));
}

fn part2(rules: &[Rule]) {
    println!("{}", Rule::count_children(rules, "shiny gold"));
}

#[derive(Debug)]
struct Rule {
    bag: Bag,
    sub_bags: Vec<Bag>
}

impl Rule {
    fn update_parents(rules: &mut [Rule], name: &str) {
        for i in 0..rules.len() {
            if rules[i].sub_bags.iter().any(|bag| bag.name == name) {
                rules[i].bag.contains_gold = true;
                Rule::update_parents(rules, &rules[i].bag.name.clone());
            }
        }
    }
    
    fn count_children(rules: &[Rule], name: &str) -> u32 {
        let mut count = 0;
        for bag in rules.iter().find(|r| r.bag.name == name).unwrap().sub_bags.iter() {
            count += bag.count * (1 + Rule::count_children(rules, &bag.name));
        }
        return count;
    }
}

#[derive(Debug)]
struct Bag {
    name: String,
    count: u32,
    contains_gold: bool
}

impl Bag {
    fn new(name: &str, count: u32) -> Self {
        Bag {
            name: String::from(name),
            count,
            contains_gold: false
        }
    }
}