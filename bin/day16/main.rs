use std::{collections::{HashMap, HashSet}, ops::RangeInclusive};

#[derive(Debug, Clone)]
struct List {
    rules: HashMap<String, Vec<RangeInclusive<u64>>>,
    mine: Vec<u64>,
    others: HashMap<usize, Vec<u64>>,
}

impl List {
    fn new() -> Self {
        Self { rules: HashMap::new(), mine: Vec::new(), others: HashMap::new() }
    }
    fn find_invalid(&mut self) -> u64 {
        let mut sum = 0;
        self.others.retain(|_, ticket| {
            let invalid_sum: u64 = ticket.iter().filter(|&&v| {
                self.rules.values().all(|ranges| ranges.iter().all(|range| !range.contains(&v)))
            }).sum();
            sum += invalid_sum;
            invalid_sum == 0
        });
        sum
    }
    fn decipher(&self) -> u64 {
        let mut poss_pos: HashMap<String, HashSet<usize>> = self.rules.keys()
            .map(|f| (f.clone(), (0..self.mine.len()).collect()))
            .collect();

        self.others.values().for_each(|t| {
            self.rules.iter().for_each(|(f, rngs)| {
                t.iter().enumerate().for_each(|(idx, &v)| {
                    if rngs.iter().all(|r| !r.contains(&v)) {
                        poss_pos.get_mut(f).unwrap().remove(&idx);
                    }
                });
            });
        });

        let mut field_pos = HashMap::new();
        while let Some((f, pos)) = poss_pos.iter().find(|(_, pos)| pos.len() == 1) {
            let &p = pos.iter().next().unwrap();
            field_pos.insert(f.clone(), p);
            poss_pos.iter_mut().for_each(|(_, p_set)| { p_set.remove(&p); });
        }

        field_pos.iter().filter_map(|(f, &p)| {
            if f.starts_with("departure") {
                Some(self.mine[p])
            } else {
                None
            }
        }).product()
    }
}

fn parse() -> Option<List> {
    let input = include_str!("input16.txt");
    let mut iter = input.split("\r\n\r\n");
    let mut list = List::new();
    for line in iter.next()?.lines() {
        let (name, v) = line.split_once(": ")?;
        let rngs: Vec<u64> = v.split(|c| c == ' ' || c == '-').filter_map(|x| x.parse().ok()).collect();
        for ch in rngs.chunks(2) {
            if let [start, end] = ch {
                list.rules.entry(name.to_string()).or_insert(Vec::new()).push(*start..=*end);
            }
        }
    }
    list.mine = iter.next()?.lines().skip(1).next()?.split(',').filter_map(|x| x.parse::<u64>().ok()).collect::<Vec<u64>>();
    for (idx, line) in iter.next()?.lines().skip(1).enumerate() {
        let n: Vec<u64> = line.split(',').filter_map(|x| x.parse().ok()).collect();
        list.others.insert(idx, n);
    }
    Some(list)
}
fn part1() -> u64 {
    let mut list = parse().unwrap();
    list.find_invalid()
}

fn part2() -> u64 {
    let mut list = parse().unwrap();
    list.find_invalid();
    list.decipher()
}
fn main() {
    println!("{}", part2());
}