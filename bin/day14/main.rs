use std::collections::HashMap;
use nom::{
    character::complete::digit1,
    bytes::complete::take_while,
    combinator::map_res,
    sequence::preceded,
    IResult,
};

const LENGTH: usize = 36;

fn parse_num(input: &str) -> IResult<&str, u64> {
    preceded(
        take_while(|c: char| !c.is_digit(10)),
        map_res(digit1, str::parse::<u64>)
    )(input)
}

fn to_u64(bits: &[u64]) -> u64 {
    bits.iter()
        .fold(0, |result, &bit| (result << 1) ^ bit as u64)
}

fn parse_bm(line: &str) -> Vec<Option<u8>> {
    line.chars().map(|c| {
        match c {
            'X' => None,
            _ => c.to_digit(10).map(|digit| digit as u8),
        }
    }).collect()
}

fn to_bin_vec(mut num: u64) -> Vec<u8> {
    let mut stack = Vec::new();

    while num > 0 {
        stack.push((num & 1) as u8);
        num >>= 1;
    }

    let mut ext = vec![0; LENGTH - stack.len()];
    while let Some(n) = stack.pop() {ext.push(n)}
    ext
}

fn apply_bm_part1(bm: &Vec<Option<u8>>, bin: Vec<u8>) -> u64 {
    let bits: Vec<u64> = bin.iter().zip(bm.iter()).map(|(&n, b)| {
        match b {
            Some(x) => *x as u64,
            None => n as u64,
        }
    }).collect();
    to_u64(&bits)
}

fn part1() -> u64 {
    let input = include_str!("input14.txt");
    let mut memory = HashMap::new();
    let mut bm = Vec::new();
    for line in input.lines() {
        if line.starts_with("mask") {
            bm = parse_bm(line.strip_prefix("mask = ").unwrap());
        } else {
            let (rest, address) = parse_num(line).unwrap();
            let (_, num) = parse_num(rest).unwrap();
            let value = apply_bm_part1(&bm, to_bin_vec(num));
            memory.insert(address, value);
        }
    }
    memory.values().sum::<u64>()
}


fn apply_bm_part2(bm: &Vec<Option<u8>>, bin: Vec<u8>) -> Vec<Option<u8>> {
    bin.iter().zip(bm.iter()).map(|(&n, b)| {
        match b {
            Some(x) if *x == 0 => Some(n),
            Some(_) => Some(1),
            None => None,
        }
    }).collect()
}

fn gen_values(bm: Vec<Option<u8>>) -> Vec<Vec<u64>> {
    let xs = bm.iter().filter(|&c| c.is_none()).count();

    (0..2u32.pow(xs as u32))
        .map(|i| {
            let mut comb = format!("{:0width$b}", i, width = xs)
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect::<Vec<_>>()
                .into_iter();

            bm.iter()
                .map(|&c| c.map(|x| x as u64).unwrap_or_else(|| comb.next().unwrap()))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn part2() -> u64 {
    let input = include_str!("input14.txt");
    let mut memory = HashMap::new();
    let mut bm = Vec::new();
    for line in input.lines() {
        if line.starts_with("mask") {
            bm = parse_bm(line.strip_prefix("mask = ").unwrap());
        } else {
            let (rest, address) = parse_num(line).unwrap();
            let (_, num) = parse_num(rest).unwrap();
            gen_values(apply_bm_part2(&bm, to_bin_vec(address))).iter().for_each(|add| {memory.insert(to_u64(add), num);})
        }
    }
    memory.values().sum::<u64>()
}

fn main() {
    println!("{}", part2());
}