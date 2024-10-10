///
/// # day_06.rs
/// Code for the day 01 of the Advent of Code challenge year 2022
///
// Imports  ==============================================================================  Imports
use std::collections::HashSet;

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../../data/year_2022/inputs/day_06.txt");

// Functions  =========================================================================== Functions
///
/// # find_marker
///
/// Find the first marker of a given size in the input string.
/// A marker is a substring with all unique characters.
///
/// ## Arguments
///
/// * `input` - The input string.
/// * `marker_size` - The size of the marker.
///
/// ## Returns
///
/// * The index of the first marker found.
fn find_marker(input: &str, marker_size: usize) -> Option<usize> {
    let chars: Vec<char> = input.chars().collect();

    // iterate over the windows of the input string
    for (i, window) in chars.windows(marker_size).enumerate() {
        // check if the window has all unique characters
        if window.iter().collect::<HashSet<_>>().len() == marker_size {
            return Some(i + marker_size);
        }
    }

    None
}

pub fn response_part_1() {
    if let Some(result) = find_marker(INPUT, 4) {
        println!("Day 06 - Part 1: {}", result);
    } else {
        println!("Day 06 - Part 1: No marker found");
    }
}

pub fn response_part_2() {
    if let Some(result) = find_marker(INPUT, 14) {
        println!("Day 06 - Part 1: {}", result);
    } else {
        println!("Day 06 - Part 1: No marker found");
    }
}
