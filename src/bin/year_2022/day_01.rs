///
/// # day_01.rs
/// Code for the day 01 of the Advent of Code challenge year 2022
///

// Imports  ==============================================================================  Imports

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../../data/year_2022/inputs/day_01.txt");

// Functions  =========================================================================== Functions
pub fn response_part_1() {
    // The input is a list of groups of numbers, each group separated by two newlines.
    // Each group is a list of numbers separated by newlines.
    //
    // We need to calculate the sum of the numbers in each group and then return the biggest sum.
    //

    let max_sum = INPUT
        .split("\n\n")
        .filter(|group| !group.is_empty())
        .map(|group| {
            group
                .split("\n")
                .filter(|group| !group.is_empty())
                .map(|num| num.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .max()
        .unwrap();

    println!("Day 01 - Part 1: {:?}", max_sum);
}

pub fn response_part_2() {
    // Here, we need the three biggests groups of numbers and return the sum of the numbers in each group.

    let max_sum = INPUT
        .split("\n\n")
        .filter(|group| !group.is_empty())
        .map(|group| {
            group
                .split("\n")
                .filter(|group| !group.is_empty())
                .map(|num| num.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();

    let mut max_sum = max_sum;
    max_sum.sort();
    max_sum.reverse();

    let result = max_sum.iter().take(3).sum::<i32>();

    println!("Day 01 - Part 2: {:?}", result);
}
