use std::fmt;
use std::iter::FromIterator;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Cell {
    Floor,
    Taken,
    Empty,
}

#[derive(Debug, Clone)]
struct Grid (Vec<Vec<Cell>>);

impl Grid {
    fn round_part1(&mut self) {
        let mut new_grid = self.0.clone();

        for y in 0..self.0.len() {
            for x in 0..self.0[0].len() {
                new_grid[y][x] = match self.0[y][x] {
                    Cell::Taken => {
                        let nbrs = self.neighbors_part1(x, y);
                        if nbrs.iter().filter(|&&s| s == Cell::Taken).count() >= 4 {
                            Cell::Empty
                        } else {
                            Cell::Taken
                        }
                    },
                    Cell::Empty => {
                        let nbrs = self.neighbors_part1(x, y);
                        if nbrs.iter().all(|&s| s != Cell::Taken) {
                            Cell::Taken
                        } else {
                            Cell::Empty
                        }
                    },
                    _ => Cell::Floor,
                };
            }
        }

        self.0 = new_grid;
    }
    fn round_part2(&mut self) {
        let mut new_grid = self.0.clone();

        for y in 0..self.0.len() {
            for x in 0..self.0[0].len() {
                new_grid[y][x] = match self.0[y][x] {
                    Cell::Taken => {
                        let nbrs = self.neighbors_part2(x, y);
                        if nbrs.iter().filter(|&&s| s == Cell::Taken).count() >= 5 {
                            Cell::Empty
                        } else {
                            Cell::Taken
                        }
                    },
                    Cell::Empty => {
                        let nbrs = self.neighbors_part2(x, y);
                        if nbrs.iter().all(|&s| s != Cell::Taken) {
                            Cell::Taken
                        } else {
                            Cell::Empty
                        }
                    },
                    _ => Cell::Floor,
                };
            }
        }

        self.0 = new_grid;
    }
    fn neighbors_part1(&self, x: usize, y: usize) -> Vec<Cell> {
        let mut seats = Vec::new();
        let mut add_neighbor = |dx: isize, dy: isize| {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0 && ny >= 0 && ny < self.0.len() as isize && nx < self.0[0].len() as isize && !(dx == 0 && dy == 0) {
                match &self.0[ny as usize][nx as usize] {
                    Cell::Taken => seats.push(Cell::Taken),
                    Cell::Empty => seats.push(Cell::Empty),
                    Cell::Floor => {},
                }
            }
        };
    
        for dx in -1..=1 {
            for dy in -1..=1 {
                add_neighbor(dx, dy);
            }
        }
        seats
    }
    fn neighbors_part2(&self, x: usize, y: usize) -> Vec<Cell> {
        let mut seats = Vec::new();
        let mut add_neighbor = |dx: isize, dy: isize| {
            let mut nx = x as isize + dx;
            let mut ny = y as isize + dy;
            while nx >= 0 && ny >= 0 && ny < self.0.len() as isize && nx < self.0[0].len() as isize {
                match &self.0[ny as usize][nx as usize] {
                    Cell::Taken => {
                        seats.push(Cell::Taken);
                        break;
                    },
                    Cell::Empty => {
                        seats.push(Cell::Empty);
                        break;
                    },
                    Cell::Floor => {
                        nx += dx;
                        ny += dy;
                    },
                }
            }
        };
    
        for dx in -1..=1 {
            for dy in -1..=1 {
                if !(dx == 0 && dy == 0) {
                    add_neighbor(dx, dy);
                }
            }
        }
        seats
    }
    fn count_Taken_seats(&self) -> usize {
        self.0.iter().flatten().filter(|&&s| s == Cell::Taken).count()
    }
    fn flat_copy(&self) -> Vec<Cell> {
        self.0.iter().flatten().map(|x| x.clone()).collect::<Vec<Cell>>()
    }    
}

impl FromIterator<Vec<Cell>> for Grid {
    fn from_iter<I: IntoIterator<Item = Vec<Cell>>>(iter: I) -> Self {
        let cells = iter.into_iter().collect();
        Grid(cells)
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.0 {
            for cell in row {
                let symbol = match cell {
                    Cell::Floor => '.',
                    Cell::Taken => '#',
                    Cell::Empty => 'L',
                };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse() -> Grid {
    let grid: Grid = include_str!("input11.txt").lines().map(|line| {
        line.chars().map(|c| {
            match c {
                '.' => Cell::Floor,
                '#' => Cell::Taken,
                _ => Cell::Empty,
            }
        }).collect::<Vec<_>>()
    }).collect();
    grid
}
fn part1() -> usize {
    let mut grid = parse();
    let mut cache = grid.flat_copy();
    loop {
        grid.round_part1();
        let copy = grid.flat_copy();
        if cache == copy {break}
        cache = grid.flat_copy();
    }
    grid.count_Taken_seats()
}
fn part2() -> usize {
    let mut grid = parse();
    let mut cache = grid.flat_copy();
    loop {
        grid.round_part2();
        let copy = grid.flat_copy();
        if cache == copy {break}
        cache = grid.flat_copy();
    }
    grid.count_Taken_seats()
}
fn main() {
    println!("{}", part2());
}