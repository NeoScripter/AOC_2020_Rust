use std::collections::HashSet;
fn rsplit_at(slice: &str, at: usize) -> (&str, &str) {
    let len = slice.len();
    slice.split_at(len - at)
}

fn part2(input: &str) -> usize {
    let hair_color = "abcdef0123456789".chars().collect::<HashSet<_>>();
    let eye_color = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().collect::<HashSet<_>>();

    input
        .split("\r\n\r\n")
        .filter(|&line| {
            line.split_whitespace().filter(|par| {
                let (key, value) = par.split_once(":").unwrap();
                match key {
                    "byr" => (1920..=2002).contains(&value.parse().unwrap_or(0)),
                    "iyr" => (2010..=2020).contains(&value.parse().unwrap_or(0)),
                    "eyr" => (2020..=2030).contains(&value.parse().unwrap_or(0)),
                    "hgt" => {
                        let (num_str, unit) = rsplit_at(value, 2);
                        match unit {
                            "cm" => (150..=193).contains(&num_str.parse().unwrap_or(0)),
                            "in" => (59..=76).contains(&num_str.parse().unwrap_or(0)),
                            _ => false,
                        }
                    }
                    "hcl" => value.starts_with('#') && value[1..].chars().all(|c| hair_color.contains(&c)),
                    "ecl" => eye_color.contains(&value),
                    "pid" => value.len() == 9 && value.chars().all(char::is_numeric),
                    _ => false,
                }
            }).count() == 7
        })
        .count()
}
fn part1(input: &str) -> usize {
    let passport_data = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    input
        .split("\r\n\r\n")
        .filter(|line| {
            passport_data
                .iter()
                .all(|&field| line.contains(field))
        })
        .count()
}

fn main() {
    let input = include_str!("input4.txt");
    println!("{}, {}", part1(input), part2(input));
}