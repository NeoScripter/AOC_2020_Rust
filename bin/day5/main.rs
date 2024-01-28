const ROWS: usize = 128;
const COL: usize = 8;

fn narrow_range(ins: char, min: &mut usize, max: &mut usize) {
    let mid = (min.clone() + max.clone()) / 2;
    match ins {
        'F' | 'L' => *max = mid,
        'B' | 'R' => *min = mid + 1,
        _ => panic!("Invalid input"),
    }
}
fn split_ret_left(v: Vec<u32>) -> Vec<u32> {
    let mid = v.len() / 2;
    let (left, _right) = v.split_at(mid);
    left.to_vec()
}
fn split_ret_right(v: Vec<u32>) -> Vec<u32> {
    let mid = v.len() / 2;
    let (_left, right) = v.split_at(mid);
    right.to_vec()
}
fn part1(input: &str) -> u32 {
    input.lines().map(|line| {
        let (rows_ins, columns_ins) = line.split_at(7);
        let mut rows: Vec<u32> = vec![0; ROWS];
        for (idx, row) in rows.iter_mut().enumerate() {
            *row = idx as u32
        }
        let mut columns: Vec<u32> = vec![0; COL];
        for (idx, column) in columns.iter_mut().enumerate() {
            *column = idx as u32
        }
        for ins in rows_ins.chars() {
            rows = match ins {
                'F' => split_ret_left(rows),
                'B' => split_ret_right(rows),
                _ => panic!("Invalid input"),
            };
        }
        let row = rows[0];
        for ins in columns_ins.chars() {
            columns = match ins {
                'L' => split_ret_left(columns),
                'R' => split_ret_right(columns),
                _ => panic!("Invalid input"),
            };
        }
        let column = columns[0];
        let seat_id = row * 8 + column;
        seat_id
    }).max().unwrap()
}
fn part2(input: &str) -> usize {
    let mut grid: Vec<Vec<u32>> = vec![vec![0; ROWS]; COL];
    input.lines().for_each(|line| {
        let (rows_ins, cols_ins) = line.split_at(7);

        let (mut row_min, mut row_max) = (0, ROWS - 1);
        for ins in rows_ins.chars() {
            narrow_range(ins, &mut row_min, &mut row_max);
        }

        let (mut col_min, mut col_max) = (0, COL - 1);
        for ins in cols_ins.chars() {
            narrow_range(ins, &mut col_min, &mut col_max);
        }
        grid[col_min][row_min] += 1;
    });
    let mut copy = grid.clone();
    let mut my_seat = 0;
    for (idxc, col) in grid.iter().enumerate() {
        for (idxr, row) in col.iter().enumerate() {
            if idxr < 15 || idxr > ROWS - 15 {
                copy[idxc][idxr] += 1;
                continue;
            }
            if *row == 0 {
                my_seat = idxr * 8 + idxc;
            }
        }
    }
    my_seat
}
fn main() {
    let input = include_str!("input5.txt");
    println!("{}, {}", part1(input), part2(input));
}