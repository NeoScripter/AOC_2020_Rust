fn part1(input: &str) -> usize {
    let count = input.lines().filter(|line| {
        let (range, rest) = line.trim().split_once(" ").unwrap();
        let (letter, password) = rest.split_once(": ").unwrap();
        let (low, high) = range.split_once("-").unwrap();
        let low: usize = low.parse().unwrap();
        let high: usize = high.parse().unwrap();

        let count = password.matches(letter).count();
        count >= low && count <= high
    }).count();
    count
}
fn part2(input: &str) -> usize {
    let count = input
        .lines()
        .filter(|line| {
            let (range, rest) = line.trim().split_once(' ').unwrap();
            let (letter, password) = rest.split_once(": ").unwrap();
            let (low, high) = range.split_once('-').unwrap();
            let (low, high) = (low.parse::<usize>().unwrap() - 1, high.parse::<usize>().unwrap() - 1);
            let letter_byte = letter.as_bytes()[0];
            let password_bytes = password.as_bytes();
            (password_bytes[low] == letter_byte) ^ (password_bytes[high] == letter_byte)
        })
        .count();
    count
}
fn main() {
    let input = include_str!("input2.txt");
    println!("{}, {}", part1(input), part2(input));
}