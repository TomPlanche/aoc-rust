///
/// # day_04.rs
/// Code for the day 01 of the Advent of Code challenge year 2015
///

// Imports  ==============================================================================  Imports
use md5::{compute};
// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../../data/year_2015/inputs/day_04.txt");

// Functions  =========================================================================== Functions
pub fn response_part_1() {
	println!("Day 04 - Part 1");

	const INPUT: &str = "yzbqklnj";

	let mut i = 0;
	loop {
		let input = format!("{}{}", INPUT, i);
		let digest = compute(input.clone());
		let hex = format!("{:x}", digest);
		if hex.starts_with("00000") {
			println!("{} - {} - {}", input, hex, i);
			break;
		}
		i += 1;
	}
}

pub fn response_part_2() {
	println!("Day 04 - Part 2");

	const INPUT: &str = "yzbqklnj";

	let mut i = 0;
	loop {
		let input = format!("{}{}", INPUT, i);
		let digest = compute(input.clone());
		let hex = format!("{:x}", digest);
		if hex.starts_with("000000") {
			println!("{} - {} - {}", input, hex, i);
			break;
		}
		i += 1;
	}
}

