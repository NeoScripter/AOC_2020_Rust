use itertools::Itertools;

fn part1(numbers: &Vec<u64>) -> u64 {
    for window in numbers.windows(26) {
        let (last, elements) = window.split_last().unwrap();
        let is_sum = elements.iter().any(|&x| {
            elements.iter().any(|&y| x != y && x + y == *last)
        });

        if !is_sum {
            return *last;
        }
    }
    panic!("No numbers matching the criteria");
}
fn part2(input: &str) -> u64 {
    let numbers: Vec<u64> = input.lines().map(|line| line.trim().parse().unwrap()).collect();
    let invalid_number = part1(&numbers);
    let mut window_size = 1;
    loop {
        window_size += 1;
        for window in numbers.windows(window_size) {
            if window.iter().sum::<u64>() == invalid_number {
                let (min, max) = window.iter().minmax().into_option().unwrap();
                return min + max
            }
        }
    }
}

fn main() {
    let input = include_str!("input9.txt");
    println!("{}", part2(input));
}