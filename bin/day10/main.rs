
fn part1(input: &str) -> u32 {
    let mut adapters: Vec<u32> = input.lines()
        .map(|l| l.parse().expect("Invalid digit"))
        .collect();
    adapters.sort_unstable();

    let last = *adapters.last().expect("vector is empty") + 3;
    adapters.push(last);

    let (_, ones, threes) = adapters.iter().fold((0, 0, 0), |(acc, ones, threes), &next| {
        match next.abs_diff(acc) {
            1 => (next, ones + 1, threes),
            3 => (next, ones, threes + 1),
            _ => (next, ones, threes)
        }
    }); 
    ones * threes
}

fn part2(input: &str) -> u64 {
    let mut adapters: Vec<u32> = input.lines()
        .map(|l| l.parse().expect("Invalid digit"))
        .collect();
    adapters.push(0);
    adapters.sort_unstable();
    adapters.push(adapters.last().unwrap() + 3);

    let mut ways: Vec<u64> = vec![0; adapters.len()];
    ways[0] = 1;

    for i in 0..adapters.len() {
        for j in (i + 1)..adapters.len() {
            if adapters[j] - adapters[i] <= 3 {
                ways[j] += ways[i];
            } else {
                break;
            }
        }
    }
    *ways.last().unwrap()
}

fn main() {
    let input = include_str!("input10.txt");
    println!("{}", part2(input));
}