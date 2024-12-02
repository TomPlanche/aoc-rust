///
/// # day_01.rs
/// Code for the day 01 of the Advent of Code challenge year 2015
///

// Imports  ==============================================================================  Imports

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../../data/year_2015/inputs/day_01.txt");
// Functions  =========================================================================== Functions
pub fn response_part_1() {
    // Count the number of open and close parenthesis
    let mut open_parenthesis: i32 = 0;
    let mut close_parenthesis: i32 = 0;
    for c in INPUT.chars() {
        match c {
            '(' => open_parenthesis += 1,
            ')' => close_parenthesis += 1,
            _ => (),
        }
    }

    // Print the result
    println!("Day 01 - Part 1: {}", open_parenthesis - close_parenthesis);
}

pub fn response_part_2() {
    println!("Day 01 - Part 2");

    // Count the number of open and close parenthesis
    let mut open_parenthesis: i32 = 0;
    let mut close_parenthesis: i32 = 0;

    for (i, c) in INPUT.chars().enumerate() {
        match c {
            '(' => open_parenthesis += 1,
            ')' => close_parenthesis += 1,
            _ => (),
        }

        if open_parenthesis - close_parenthesis == -1 {
            print!("The position is: {}", i + 1);
            break;
        }
    }
}
