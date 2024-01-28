use std::collections::{HashSet, HashMap};

fn part1(input: &str) -> u32 {
    input.split("\r\n\r\n").map(|group| {
        let mut set = HashSet::new();
        group.lines().for_each(|pas| {
            pas.chars().for_each(|c| {
                set.insert(c);
            });
        });
        set.len() as u32
    })
    .sum()
}
fn part2(input: &str) -> usize {
    input
        .split("\r\n\r\n")
        .map(|group| {
            let person_count = group.lines().count();
            let mut answer_counts = HashMap::new();
            group.lines().flat_map(str::chars).for_each(|answer| {
                *answer_counts.entry(answer).or_insert(0) += 1;
            });
            answer_counts.values().filter(|&&count| count == person_count).count()
        })
        .sum()
}
fn main() {
    let input = include_str!("input6.txt");
    println!("{}, {}", part1(input), part2(input));
}