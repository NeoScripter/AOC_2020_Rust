use std::collections::HashSet;
#[derive(Debug, Copy, Clone)]
enum Operation {
    Nop,
    Jump,
    Acc,
}
#[derive(Debug, Copy, Clone)]
struct In {
    op: Operation,
    arg: i32,
}
impl In {
    fn new(op: Operation, arg: i32) -> In {
        In {
            op: op,
            arg: arg,
        } 
    }
}
fn parse_input(input: &str) -> Vec<In> {
    let program: Vec<In> = input
    .lines()
    .map(|line| {
        let (operation, argument) = line.trim().split_once(" ").unwrap();
        let argument = argument.parse::<i32>().unwrap();
        let operation = match operation {
            "nop" => Operation::Nop,
            "acc" => Operation::Acc,
            "jmp" => Operation::Jump,
            _ => panic!("invalid input"),
        };
        let instruction = In::new(operation, argument);
        instruction
    }).collect();
    program
}
fn part1(input: &str) -> i32 {
    let program = parse_input(input);
    let mut seen = HashSet::new();
    let mut index: i32 = 0;
    let mut acc = 0;
    loop {
        let current = &program[index as usize];
        let value = current.arg;
        let key = current.op;
        if !seen.insert(index) {
            return acc;
        }
        match key {
            Operation::Nop => index += 1,
            Operation::Acc => {
                index += 1;
                acc += value;
            },
            Operation::Jump => index += value,
        }
    }
}
fn part2(input: &str) -> i32 {
    let program = parse_input(input); 
    let length = program.len();
    'outer: for idx in 0..length { 
    let mut modified_program = program.clone();
    modified_program[idx].op = match modified_program[idx].op {
        Operation::Nop => Operation::Jump,
        Operation::Jump => Operation::Nop,
        _ => continue,
    };
    let mut seen = HashSet::new();
    let mut index: i32 = 0;
    let mut acc = 0;
        loop {
            if index as usize == length {
                return acc;
            }
            let current = modified_program[index as usize];
            let value = current.arg;
            let key = current.op;
            if !seen.insert(index) {
                continue 'outer;
            }
            match key {
                Operation::Nop => index += 1,
                Operation::Acc => {
                    index += 1;
                    acc += value;
                },
                Operation::Jump => index += value,
            }
        }
    }
    0
}

fn main() {
    let input = include_str!("input8.txt");
    println!("{}", part2(input));    
}