use itertools::Itertools;
fn part1(input: &str) -> u32 {
    let lines: Vec<u32> = input.lines().filter_map(|line| line.trim().parse().ok()).collect();
    let product = lines.iter()
        .permutations(2)
        .find(|p| p.iter().copied().sum::<u32>() == 2020)
        .map_or(0, |perm| perm.iter().copied().product());
    product
}
fn part2(input: &str) -> u32 {
    let lines: Vec<u32> = input.lines().filter_map(|line| line.trim().parse().ok()).collect();
    let product = lines.iter()
        .permutations(3)
        .find(|p| p.iter().copied().sum::<u32>() == 2020)
        .map_or(0, |perm| perm.iter().copied().product());
    product
}
fn main() {
    let input = include_str!("input1.txt");
    println!("{}, {}", part1(input), part2(input));
}