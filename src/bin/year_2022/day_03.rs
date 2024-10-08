///
/// # day_03.rs
/// Code for the day 01 of the Advent of Code challenge year 2022
///
// Imports  ==============================================================================  Imports
use std::collections::HashSet;

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../../data/year_2022/inputs/day_03.txt");

// Functions  =========================================================================== Functions
///
/// # get_priority
///
/// Get the priority of a character
/// The priority is calculated as follows:
/// - a..z: 1..26
/// - A..Z: 27..52
///
/// ## Arguments
///
/// * `c` - The character to get the priority
///
/// ## Returns
///
/// * `u32` - The priority of the character
fn get_priority(c: char) -> u32 {
    match c {
        // chars from a..z
        'a'..='z' => c as u32 - 'a' as u32 + 1,

        // chars from A..Z
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => 0,
    }
}

pub fn response_part_1() {
    println!("Day 03 - Part 1");

    let total: u32 = INPUT
        .lines()
        .map(|line| {
            let mid = line.len() / 2;

            // split the line in two parts
            let first: HashSet<char> = line[..mid].chars().collect();
            let second: HashSet<char> = line[mid..].chars().collect();

            // get the common character
            let common = first.intersection(&second).next().unwrap();

            // get the priority of the common character
            get_priority(*common)
        })
        .sum();

    println!("Total: {}", total);
}

pub fn response_part_2() {
    println!("Day 03 - Part 2");

    let total: u32 = INPUT
        .lines()
        .collect::<Vec<&str>>() // collect the lines in a vector
        .chunks(3) // split the vector in chunks of 3
        .map(|group| {
            // get the sets of each rucksack
            let sets: Vec<HashSet<char>> = group
                .iter()
                .map(|rucksack| rucksack.chars().collect())
                .collect();

            // get the common item in the sets
            let common_item = sets[0]
                .iter()
                .find(|&&item| sets[1].contains(&item) && sets[2].contains(&item))
                .unwrap();

            // get the priority of the common item
            get_priority(*common_item)
        })
        .sum();

    println!("Total: {}", total);
}
