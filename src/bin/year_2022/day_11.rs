///
/// # day_11.rs
/// Code for the day 01 of the Advent of Code challenge year 2022
///
// Imports  ==============================================================================  Imports
use std::str::FromStr;

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../../data/year_2022/inputs/day_11.txt");

// #[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    test: u64,
    if_true: usize,
    if_false: usize,
    inspections: u64,
    common_modulus: u64, // for part 2
}

impl FromStr for Monkey {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let items = lines[1]
            .split(": ")
            .nth(1)
            .unwrap()
            .split(", ")
            .map(|n| n.parse().unwrap())
            .collect();

        let operation_parts: Vec<&str> = lines[2].split_whitespace().collect();
        let operation: Box<dyn Fn(u64) -> u64> = match (operation_parts[4], operation_parts[5]) {
            ("+", "old") => Box::new(|x| x + x),
            ("*", "old") => Box::new(|x| x * x),
            ("+", n) => {
                let num: u64 = n.parse().unwrap();
                Box::new(move |x| x + num)
            }
            ("*", n) => {
                let num: u64 = n.parse().unwrap();
                Box::new(move |x| x * num)
            }
            _ => panic!("Unknown operation"),
        };

        let test: u64 = lines[3].split_whitespace().last().unwrap().parse()?;
        let if_true: usize = lines[4].split_whitespace().last().unwrap().parse()?;
        let if_false: usize = lines[5].split_whitespace().last().unwrap().parse()?;

        Ok(Monkey {
            items,
            operation,
            test,
            if_true,
            if_false,
            inspections: 0,
            common_modulus: 0,
        })
    }
}
// Functions  =========================================================================== Functions
///
/// # simulate_rounds
///
/// Simulate the rounds of the monkeys throwing items.
///
/// ## Arguments
///
/// * `monkeys` - A mutable reference to a vector of monkeys.
/// * `rounds` - The number of rounds to simulate.
fn simulate_rounds(monkeys: &mut Vec<Monkey>, rounds: usize) {
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let mut throws = Vec::new();

            // separated scope to avoid borrowing issues
            {
                let monkey = &mut monkeys[i];

                while let Some(item) = monkey.items.pop() {
                    monkey.inspections += 1;

                    let worry = (monkey.operation)(item) / 3;

                    let target = if worry % monkey.test == 0 {
                        monkey.if_true
                    } else {
                        monkey.if_false
                    };

                    throws.push((target, worry));
                }
            }

            for (target, item) in throws {
                monkeys[target].items.push(item);
            }
        }
    }
}

fn simulate_rounds_2(monkeys: &mut Vec<Monkey>, rounds: usize) {
    let common_modulus: u64 = monkeys.iter().map(|m| m.test).product();
    for monkey in monkeys.iter_mut() {
        monkey.common_modulus = common_modulus;
    }

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let mut throws = Vec::new();
            {
                let monkey = &mut monkeys[i];
                while let Some(item) = monkey.items.pop() {
                    monkey.inspections += 1;
                    let worry = (monkey.operation)(item) % monkey.common_modulus;
                    let target = if worry % monkey.test == 0 {
                        monkey.if_true
                    } else {
                        monkey.if_false
                    };
                    throws.push((target, worry));
                }
            }
            for (target, item) in throws {
                monkeys[target].items.push(item);
            }
        }
    }
}

pub fn response_part_1() {
    let mut monkeys: Vec<Monkey> = INPUT.split("\n\n").map(|s| s.parse().unwrap()).collect();
    simulate_rounds(&mut monkeys, 20);

    let mut inspection_counts: Vec<u64> = monkeys.iter().map(|m| m.inspections).collect();
    inspection_counts.sort_unstable_by(|a, b| b.cmp(a));

    let monkey_business = inspection_counts[0] * inspection_counts[1];
    println!("Day 11 - Part 1: {}", monkey_business);
}

pub fn response_part_2() {
    let mut monkeys: Vec<Monkey> = INPUT.split("\n\n").map(|s| s.parse().unwrap()).collect();
    simulate_rounds_2(&mut monkeys, 10000);

    let mut inspection_counts: Vec<u64> = monkeys.iter().map(|m| m.inspections).collect();
    inspection_counts.sort_unstable_by(|a, b| b.cmp(a));

    let monkey_business = inspection_counts[0] * inspection_counts[1];
    println!("Day 11 - Part 2: {}", monkey_business);
}
