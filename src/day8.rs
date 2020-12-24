use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/day8.txt").expect("Failed to read file");
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut console = Console::new(input);
    while console.is_new_instr() {
        console.tick();
    }
    println!("{}", console.acc);
}

fn part2(input: &str) {
    let mut console = Console::new(input);
    let mut to_change: Vec<usize> = Vec::new();
    for (i, instr) in console.instr_mem.iter().enumerate() {
        if instr.op == "jmp" || instr.op == "nop" {
            to_change.push(i);
        }
    }
    while console.pc < console.instr_mem.len() && !to_change.is_empty() {
        console = Console::new(input);
        let instr = &mut console.instr_mem[to_change.pop().unwrap()];
        if instr.op == "jmp" {
            instr.op = String::from("nop");
        } else if instr.op == "nop" {
            instr.op = String::from("jmp");
        }
        while console.pc < console.instr_mem.len() && console.is_new_instr() {
            console.tick();
        }
    }
    println!("{}", console.acc);
}

struct Instruction {
    op: String,
    arg: i32,
    is_new: bool
}

struct Console {
    acc: i32,
    pc: usize,
    instr_mem: Vec<Instruction>
}

impl Console {
    fn new(input: &str) -> Self {
        let instr_mem: Vec<Instruction> = input.lines().map(|line| Instruction {
            op: String::from(&line[0..3]),
            arg: line[4..].parse::<i32>().unwrap(),
            is_new: true
        }).collect();
        Console {
            acc: 0,
            pc: 0,
            instr_mem
        }
    }

    fn is_new_instr(&self) -> bool {
        self.instr_mem[self.pc].is_new
    }

    fn tick(&mut self) {
        let instr = &mut self.instr_mem[self.pc];
        instr.is_new = false;
        let op = instr.op.as_str();
        let arg = instr.arg;
        match op {
            "acc" => self.acc(arg),
            "jmp" => self.jmp(arg),
            _ => self.nop()
        }
    }

    fn acc(&mut self, arg: i32) {
        self.acc += arg;
        self.pc += 1;
    }

    fn jmp(&mut self, arg: i32) {
        if arg < 0 {
            self.pc -= -arg as usize;
        } else {
            self.pc += arg as usize;
        }
    }

    fn nop(&mut self) {
        self.pc += 1;
    }
}