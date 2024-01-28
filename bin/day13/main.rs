use std::{
    collections::HashMap,
};
use num_integer::lcm;

#[derive(Debug)]
enum Bus {
    Id(u64),
    X,
}

#[derive(Debug)]
struct Station {
    timestamp: u64,
    buses: Vec<Bus>,
    wait_times: HashMap<u64, u64>,
    offsets: Vec<u64>,
    ids: Vec<u64>,
}

impl Station {
    fn new(timestamp: u64) -> Self {
        Station {
            timestamp,
            buses: Vec::new(),
            wait_times: HashMap::new(),
            offsets: Vec::new(),
            ids: Vec::new(),
        }
    }
    fn populate(&mut self) {
        for (idx, bus) in self.buses.iter().enumerate() {
            match bus {
                Bus::Id(x) => {
                    let last_arrival = self.timestamp - (self.timestamp % x);
                    let wait_time = last_arrival + x - self.timestamp;
                    self.wait_times.insert(wait_time,*x);
                    self.offsets.push(idx as u64);
                    self.ids.push(*x);
                },
                _ => {},
            }
        }
    }
    fn answer_part1(&self) -> u64 {
        let min_time = self.wait_times.keys().min().unwrap();
        self.wait_times.get(min_time).unwrap() * min_time
    }
    fn answer_part2(&mut self) -> u64 {
        let mut timestamp = 0;
        let mut step = 1;

        for (id, offset) in self.ids.iter().zip(self.offsets.iter()) {
            while (timestamp + offset) % id != 0 {
                timestamp += step;
            }
            step = lcm(step, *id);
        }
    
        timestamp
    }
}

fn part2(input: &str) -> u64 {
    let (timestamp, buses) = input.split_once("\r\n").unwrap();
    let timestamp = timestamp.parse::<u64>().unwrap();
    let mut station = Station::new(timestamp);
    buses.split(',').for_each(|bus| {
        match bus {
            "x" => station.buses.push(Bus::X),
            _ => station.buses.push(Bus::Id(bus.parse::<u64>().unwrap())),
        }
    });
    station.populate();
    station.answer_part2()
}
fn main() {
    let input = include_str!("input13.txt");
    println!("{}", part2(input));
}