use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Game {
    last_num: u32,
    mem: HashMap<u32, usize>,
    turn: usize,
}

impl Game {
    fn new(input: Vec<u32>) -> Self {
        let mut mem = HashMap::new();
        let last_num = *input.last().unwrap();
        for (i, &num) in input.iter().enumerate() {
            mem.insert(num, i + 1);
        }
        Self { last_num, mem, turn: input.len() }
    }

    fn play(&mut self, rnds: usize) -> u32 {
        self.mem.reserve(1000000); 
    
        while self.turn < rnds {
            let last_num = self.last_num;
            self.last_num = match self.mem.get(&last_num) {
                Some(&last_seen) => (self.turn - last_seen) as u32,
                None => 0,
            };
            *self.mem.entry(last_num).or_default() = self.turn;
            self.turn += 1;
        }
        self.last_num
    }
    
}
fn part1(input: Vec<u32>) -> u32 {
    let mut game = Game::new(input);
    game.play(2020)
}

fn part2(input: Vec<u32>) -> u32 {
    let mut game = Game::new(input);
    game.play(30000000)
}

fn main() {
    let input = vec![11,0,1,10,5,19];
    println!("{}", part1(input));
}