#![allow(unused)]
#![allow(dead_code)]
#![allow(unused_variables)]
use itertools::Itertools;
use std::{collections::{HashMap, HashSet}, ops::RangeInclusive};
use std::time::Instant;

#[derive(Debug, Clone)]
struct List {
    rules: HashMap<usize, Vec<RangeInclusive<u32>>>,
    mine: Vec<u32>,
    others: HashMap<usize, Vec<u32>>,
}

impl List {
    fn new() -> Self {
        Self { rules: HashMap::new(), mine: Vec::new(), others: HashMap::new() }
    }
    fn find_invalid(&mut self) -> u32 {
        let mut sum = 0;
        self.others.retain(|&i, ticket| {
            let invalid_sum: u32 = ticket.iter().filter(|&&v| {
                self.rules.values().all(|ranges| ranges.iter().all(|range| !range.contains(&v)))
            }).sum();
            sum += invalid_sum;
            invalid_sum == 0
        });
        sum
    }
    fn decipher(&self) -> u32 {
        let mut used_positions = HashSet::new();
        let mut product = 1;
    
        for i in 0..3 {
            'outer: for n in 0..self.mine.len() {
                for k in 0..self.others.len() {
                    if let Some(ticket) = self.others.get(&k) {
                        if self.rules[&i].iter().all(|range| !range.contains(&ticket[n])) {
                            continue 'outer;
                        }
                    }
                }
                used_positions.insert(n);
                product *= self.mine[n];
                break;
            }
        }
        product
    }
}

fn parse() -> Option<List> {
    let input = include_str!("input_lib.txt");
    let mut iter = input.split("\r\n\r\n");
    let mut list = List::new();
    for (idx, line) in iter.next()?.lines().enumerate() {
        let (_, v) = line.split_once(": ")?;
        let rngs: Vec<u32> = v.split(|c| c == ' ' || c == '-').filter_map(|x| x.parse().ok()).collect();
        for ch in rngs.chunks(2) {
            if let [start, end] = ch {
                list.rules.entry(idx).or_insert(Vec::new()).push(*start..=*end);
            }
        }
    }
    list.mine = iter.next()?.lines().skip(1).next()?.split(',').filter_map(|x| x.parse::<u32>().ok()).collect::<Vec<u32>>();
    for (idx, line) in iter.next()?.lines().skip(1).enumerate() {
        let n: Vec<u32> = line.split(',').filter_map(|x| x.parse().ok()).collect();
        list.others.insert(idx, n);
    }
    Some(list)
}
fn part1() -> u32 {
    let now = Instant::now();
    let mut list = parse().unwrap();
    list.find_invalid()
}

fn part2() -> u32 {
    let now = Instant::now();
    let mut list = parse().unwrap();
    list.find_invalid();
    list.decipher()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(71, part1());
    }
    #[test]
    fn test_2() {
        assert_eq!(1716, part2());
    }
}