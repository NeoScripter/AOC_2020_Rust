use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::cell::RefCell;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::multispace0,
    combinator::{map, opt},
    multi::separated_list0,
    sequence::{terminated, tuple},
    IResult,
};
#[derive(Debug)]
struct Bag {
    name: String,
    content: HashMap<String, u32>,
    children: Vec<Rc<RefCell<Bag>>>,
}

impl PartialEq for Bag {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Hash for Bag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Bag {
    fn new(name: &str) -> Bag {
        Bag {
            name: String::from(name),
            content: HashMap::new(),
            children: Vec::new(),
        }
    }

    fn print_tree(&self, depth: usize) {
        let indent = " ".repeat(depth * 2);
        println!("{}Bag: {}", indent, self.name);
        for (child_name, quantity) in &self.content {
            println!("{}  Contains: {} x {}", indent, quantity, child_name);
        }
        for child in &self.children {
            child.borrow().print_tree(depth + 1);
        }
    }
    fn add_bag(&mut self, bag: &Rc<RefCell<Bag>>) {
        self.children.push(Rc::clone(bag));
    }
    fn dist(&self, target: &str) -> bool {
        if self.name == target {
            return true;
        }
        for child in &self.children {
            if child.borrow().dist(target) {
                return true;
            }
        }
        false
    }
    fn count_total_bags(&self) -> u32 {
        let mut total = 0;

        for child in &self.children {
            let quantity = self.content.get(&child.borrow().name).unwrap_or(&0);

            total += quantity * (1 + child.borrow().count_total_bags());
        }

        total
    }
}
fn parse(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
    let (input, bag_type) = terminated(take_until(" bags"), tag(" bags"))(input)?;
    let (input, _) = tag(" contain ")(input)?;

    let (input, contents) = separated_list0(
        tag(", "),
        map(
            terminated(
                take_until(" bag"),
                tuple((tag(" bag"), opt(tag("s")), opt(tag(".")), multispace0)),
            ),
            |desc: &str| desc.trim_start_matches(|c: char| !c.is_digit(10)),
        )
    )(input)?;
    Ok((input, (bag_type, contents)))
}
fn process(input: &str) -> (HashSet<&str>, HashMap<&str, Rc<RefCell<Bag>>>) {
    let mut bag_map: HashMap<&str, Rc<RefCell<Bag>>> = HashMap::new();
    let mut all_bags: HashSet<&str> = HashSet::new();

    for line in input.lines() {
        if let Ok((_, (bag_type, contents))) = parse(line) {
            all_bags.insert(bag_type);  
            bag_map.entry(bag_type).or_insert(Rc::new(RefCell::new(Bag::new(bag_type))));
            for content in contents {
                if let Some((quantity, color)) = content.split_once(" ") {
                    let q = quantity.parse::<u32>().unwrap();
                    bag_map.entry(color).or_insert(Rc::new(RefCell::new(Bag::new(color)))); 
                    let bag_type = bag_map.get(&bag_type).unwrap();
                    bag_type.borrow_mut().content.insert(color.to_string(), q);
                    let color = bag_map.get(&color).unwrap();
                    bag_type.borrow_mut().add_bag(color);
                }
            }
        }
    }
    (all_bags, bag_map)
}
fn part1(input: &str) -> usize {
    let (mut all_bags, bag_map) = process(input);
    all_bags.remove("shiny gold");
    let answer = all_bags.into_iter().filter(|bag| {
        let root = bag_map.get(bag).unwrap();
        let max_dist = root.borrow().dist("shiny gold");
        max_dist
    }).count();
    answer
}
fn part2(input: &str) -> u32 {
    let (mut all_bags, bag_map) = process(input);

    let root = bag_map.get("shiny gold").unwrap();
    root.borrow().print_tree(1);
    let total_bags = root.borrow().count_total_bags();
    total_bags
}
fn main() {
    let input = include_str!("input7.txt");
    println!("{}", part2(input));
}