use std::mem;

fn part1(input: &str) -> i32 {
    let mut degree: i32 = 0;
    let (mut x, mut y) = (0, 0);
    input.lines().for_each(|line| {
        let com: Vec<&str> = line.splitn(3, "").filter(|&x| x != "").collect();
        let steps: i32 = com[1].parse().expect("Parsing error");
        match com[0] {
            "N" => y += steps,
            "S" => y -= steps,
            "E" => x += steps,
            "W" => x -= steps,
            "L" => degree += steps,
            "R" => degree -= steps,
            "F" => {
                let compass = degree.rem_euclid(360);
                match compass {
                    0 => x += steps,
                    90 => y += steps,
                    180 => x -= steps,
                    270 => y -= steps,
                    _ => println!("The number shouldn't exceed 360"),
                }
            },
            _ => println!("Invalid command"),
         }
    });
    x.abs() + y.abs()
}

fn part2(input: &str) -> i32 {
    let mut degree: i32 = 0;
    let mut ship_cds = (0, 0);
    let mut wpst_cds = (10, 1);
    input.lines().for_each(|line| {
        let com: Vec<&str> = line.splitn(3, "").filter(|&x| x != "").collect();
        let steps: i32 = com[1].parse().expect("Parsing error");
        match com[0] {
            "N" => wpst_cds.1 += steps,
            "S" => wpst_cds.1 -= steps,
            "E" => wpst_cds.0 += steps,
            "W" => wpst_cds.0 -= steps,
            "L" => match steps {
                90 => {
                    mem::swap(&mut wpst_cds.0, &mut wpst_cds.1);
                    wpst_cds.0 = -wpst_cds.0
                },
                180 => {
                    wpst_cds.0 = -wpst_cds.0;
                    wpst_cds.1 = -wpst_cds.1
                },
                _ => {
                    mem::swap(&mut wpst_cds.0, &mut wpst_cds.1);
                    wpst_cds.1 = -wpst_cds.1
                },
            }
            "R" => match steps {
                90 => {
                    mem::swap(&mut wpst_cds.0, &mut wpst_cds.1);
                    wpst_cds.1 = -wpst_cds.1
                },
                180 => {
                    wpst_cds.0 = -wpst_cds.0;
                    wpst_cds.1 = -wpst_cds.1
                },
                _ => {
                    mem::swap(&mut wpst_cds.0, &mut wpst_cds.1);
                    wpst_cds.0 = -wpst_cds.0
                },
            }
            "F" => {
                ship_cds.0 += wpst_cds.0 * steps;
                ship_cds.1 += wpst_cds.1 * steps;
            },
            _ => println!("Invalid command"),
         }
    });
    ship_cds.0.abs() + ship_cds.1.abs()
}
fn main() {
    let input = include_str!("input12.txt");
    println!("{}", part2(input));
}