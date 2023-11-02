///
/// # day_02.rs
/// Code for the day 01 of the Advent of Code challenge year 2015
///

// Imports  ==============================================================================  Imports
use std::cmp::min;

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../../data/year_2015/inputs/day_02.txt");

// Functions  =========================================================================== Functions
///
/// # calc_surface_area
/// Calculates the surface area of a box.
///
/// ## Arguments
/// * `l` - The length of the box
/// * `w` - The width of the box
/// * `h` - The height of the box
///
/// ## Returns
/// * `u32` - The surface area of the box
fn calc_surface_area(l: u32, w: u32, h: u32) -> u32 {
	2 * l * w + 2 * w * h + 2 * h * l
}

///
/// # calc_cubic_feet
/// Calculates the cubic feet of a box.
///
/// ## Arguments
/// * `l` - The length of the box
/// * `w` - The width of the box
/// * `h` - The height of the box
///
/// ## Returns
/// * `u32` - The cubic feet of the box
fn calc_cubic_feet(l: u32, w: u32, h: u32) -> u32 {
	l * w * h
}
// Main  ====================================================================================  Main
pub fn response_part_1() {
	println!("Day 02 - Part 1");

	let mut total_paper: u32 = 0;

	for line in INPUT.lines() {
		let mut dimensions: Vec<u32> = line.split('x').map(|x| x.parse::<u32>().unwrap()).collect();
		dimensions.sort();
		total_paper += calc_surface_area(dimensions[0], dimensions[1], dimensions[2]) + dimensions[0] * dimensions[1];
	}

	println!("Total paper: {}", total_paper);
}

pub fn response_part_2() {
	println!("Day 02 - Part 2");

	let mut total_ribbon: u32 = 0;

	for line in INPUT.lines() {
		let mut dimensions: Vec<u32> = line.split('x').map(|x| x.parse::<u32>().unwrap()).collect();
		dimensions.sort();
		total_ribbon += 2 * dimensions[0] + 2 * dimensions[1] + calc_cubic_feet(dimensions[0], dimensions[1], dimensions[2]);
	}

	println!("Total ribbon: {}", total_ribbon);
}

