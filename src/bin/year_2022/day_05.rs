///
/// # day_05.rs
/// Code for the day 01 of the Advent of Code challenge year 2022
///
// Imports  ==============================================================================  Imports
use std::collections::VecDeque;

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../../data/year_2022/inputs/day_05.txt");

// Functions  =========================================================================== Functions
///
/// # parse_input
///
/// Parse the input string and return a tuple with the stacks and moves.
///
/// ## Arguments
///
/// * `input` - The input string.
///
/// ## Returns
///
/// * A tuple with the stacks and moves.
fn parse_input(input: &str) -> (Vec<VecDeque<char>>, Vec<(usize, usize, usize)>) {
    let (stacks_str, moves_str) = input.split_once("\n\n").unwrap();

    // Parse stacks
    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); 9]; // Assuming max 9 stacks
    for line in stacks_str.lines().rev().skip(1) {
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c != ' ' {
                stacks[i].push_front(c);
            }
        }
    }

    // Parse moves
    let moves: Vec<(usize, usize, usize)> = moves_str
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            (
                parts[1].parse().unwrap(),
                parts[3].parse::<usize>().unwrap() - 1, // 1-indexed to 0-indexed
                parts[5].parse::<usize>().unwrap() - 1,
            )
        })
        .collect();

    (stacks, moves)
}

///
/// # execute_moves
///
/// Execute the moves on the stacks and return the resulting top crates.
///
/// ## Arguments
///
/// * `stacks` - A vector of VecDeque<char> representing the initial stacks of crates.
/// * `moves` - A slice of tuples (count, from, to) representing the moves to execute.
///
/// ## Returns
///
/// * A String containing the top crates of each stack after executing all moves.
fn execute_moves_9000(mut stacks: Vec<VecDeque<char>>, moves: &[(usize, usize, usize)]) -> String {
    for &(count, from, to) in moves {
        for _ in 0..count {
            if let Some(crate_) = stacks[from].pop_front() {
                stacks[to].push_front(crate_);
            }
        }
    }

    // This line collects the top crates from each stack into a String.
    // It iterates over the stacks, uses filter_map to get the first (top) crate of each non-empty stack,
    // and then collects these characters into a String, which represents the final arrangement.
    stacks.iter().filter_map(|stack| stack.front()).collect()
}

///
/// # execute_moves_9001
///
/// Execute the moves on the stacks using the CrateMover 9001 and return the resulting top crates.
///
/// ## Arguments
///
/// * `stacks` - A vector of VecDeque<char> representing the initial stacks of crates.
/// * `moves` - A slice of tuples (count, from, to) representing the moves to execute.
///
/// ## Returns
///
/// * A String containing the top crates of each stack after executing all moves.
fn execute_moves_9001(mut stacks: Vec<VecDeque<char>>, moves: &[(usize, usize, usize)]) -> String {
    for &(count, from, to) in moves {
        let mut temp_stack = VecDeque::new();
        for _ in 0..count {
            if let Some(crate_) = stacks[from].pop_front() {
                temp_stack.push_front(crate_);
            }
        }
        for crate_ in temp_stack {
            stacks[to].push_front(crate_);
        }
    }

    stacks.iter().filter_map(|stack| stack.front()).collect()
}

pub fn response_part_1() {
    let (stacks, moves) = parse_input(INPUT);
    let result = execute_moves_9000(stacks, &moves);
    println!("Day 05 - Part 1: {}", result);
}

pub fn response_part_2() {
    let (stacks, moves) = parse_input(INPUT);
    let result = execute_moves_9001(stacks, &moves);

    println!("Day 05 - Part 2: {}", result);
}
