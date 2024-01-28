fn part1(input: &str) -> u32 {
    let lines = input.lines().map(|line| line.trim()).collect::<Vec<_>>();
    let row_count = lines.len();
    let col_count = lines[0].len();

    let mut grid: Vec<Vec<u32>> = vec![];
    for &line in &lines {
        let mut row: Vec<u32> = vec![];
        for c in line.chars() {
            let cell_value = match c {
                '.' => 0,
                '#' => 1,
                _ => panic!("Invalid input"),
            };
            row.push(cell_value);
        }
        grid.push(row.repeat(3));
    }

    let mut x = 0;
    let mut y = 0;
    let mut trees_encountered = 0;

    while y < row_count {
        trees_encountered += grid[y][x % col_count];
        x += 3;
        y += 1;
    }
    trees_encountered
}
fn part2(input: &str) -> u32 {
    let lines = input.lines().map(|line| line.trim()).collect::<Vec<_>>();
    let row_count = lines.len();
    let col_count = lines[0].len();

    let grid: Vec<Vec<u32>> = lines
        .iter()
        .map(|&line| {
            line.chars()
                .map(|c| match c {
                    '.' => 0,
                    '#' => 1,
                    _ => panic!("Invalid input"),
                })
                .collect::<Vec<u32>>()
        })
        .collect();

    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    slopes.iter().fold(1, |acc, &(dx, dy)| {
        let trees_encountered = (0..row_count)
            .step_by(dy)
            .zip((0..).map(|i| (i * dx) % col_count))
            .filter(|&(y, x)| grid[y][x] == 1)
            .count() as u32;
        acc * trees_encountered
    })
}
fn main() {
    let input = include_str!("input3.txt");
    println!("{}, {}", part1(input), part2(input));
}