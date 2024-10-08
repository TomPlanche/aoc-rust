///
/// # day_04.rs
/// Code for the day 01 of the Advent of Code challenge year 2022
///
// Imports  ==============================================================================  Imports
use std::str::FromStr;

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../../data/year_2022/inputs/day_04.txt");

///
/// Represents a range of section IDs
///
/// ## Fields
/// * `start` - The starting section ID of the range
/// * `end` - The ending section ID of the range
struct Range {
    start: u32,
    end: u32,
}

///
/// Implements parsing from a string in the format "start-end"
///
/// ## Fields
///
/// * `start` - The starting section ID of the range
/// * `end` - The ending section ID of the range
/// Implements parsing from a string in the format "start-end"
///
/// ## Errors
/// Returns an error if the string cannot be parsed into two u32 values
impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').ok_or(())?;
        Ok(Range {
            start: start.parse().map_err(|_| ())?, // map_err(|_| ()) is equivalent to .unwrap()
            end: end.parse().map_err(|_| ())?,
        })
    }
}

///
/// Implements methods for the Range struct
impl Range {
    ///
    /// Checks if this range fully contains another range
    ///
    /// # Arguments
    /// * `other` - The other range to check against
    ///
    /// # Returns
    /// `true` if this range fully contains the other range, `false` otherwise
    fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    ///
    /// Checks if this range overlaps with another range
    ///
    /// # Arguments
    /// * `other` - The other range to check against
    ///
    /// # Returns
    /// `true` if there is any overlap between the ranges, `false` otherwise
    fn overlaps(&self, other: &Range) -> bool {
        self.start <= other.end && other.start <= self.end
    }
}

fn parse_input(input: &str) -> Vec<(Range, Range)> {
    input
        .lines()
        .filter_map(|line| {
            let (first, second) = line.split_once(',')?;
            Some((first.parse().ok()?, second.parse().ok()?))
        })
        .collect()
}

// Functions  =========================================================================== Functions
pub fn response_part_1() {
    println!("Day 04 - Part 1");

    let pairs = parse_input(INPUT);
    let count = pairs
        .iter()
        // Filter out pairs where one range fully contains the other
        .filter(|(a, b)| a.contains(b) || b.contains(a))
        .count();

    println!("Number of pairs with full containment: {}", count);
}

pub fn response_part_2() {
    println!("Day 04 - Part 2");

    let pairs = parse_input(INPUT);
    let count = pairs
        .iter()
        // Filter out pairs where there is no overlap
        .filter(|(a, b)| a.overlaps(b))
        .count();

    println!("Number of pairs with any overlap: {}", count);
}
