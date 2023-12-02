use std::collections::HashMap;

///
/// # day_01.rs
/// Code for the day 01 of the Advent of Code challenge year 2023
///

// Imports  ==============================================================================  Imports


// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../../data/year_2023/inputs/day_01.txt");

// Functions  =========================================================================== Functions
pub fn response_part_1() {
	println!("Day 01 - Part 1");

	let sum: u32 = INPUT
		.lines()
		.map(|line| {
			let mut bytes = line.bytes();
			let first = bytes.find(u8::is_ascii_digit).unwrap();
			let last = bytes.rev().find(u8::is_ascii_digit).unwrap_or(first);
			u32::from((first - b'0') * 10 + (last - b'0'))
		}).sum();

	println!("Sum: {}", sum);
}


const NAMED_NUMBERS: [&[u8]; 9] = [
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

#[inline(always)]
fn find_digit((idx, b): (usize, &u8), line: &[u8]) -> Option<u32> {
    b.is_ascii_digit().then(|| u32::from(b - b'0')).or_else(|| {
        NAMED_NUMBERS
            .iter()
            .position(|&name| line[idx..].starts_with(name))
            .map(|i| i as u32 + 1)
    })
}

pub fn response_part_2() {
	println!("Day 01 - Part 2");

	let sum: u32 = INPUT
		.lines()
		.map(|l| l.as_bytes())
		.map(|line| {
			let mut bytes = line.iter().enumerate();
			let first = bytes.find_map(|byte| find_digit(byte, line)).unwrap();
			let last = bytes
				.rev()
				.find_map(|byte| find_digit(byte, line))
				.unwrap_or(first);

			first * 10 + last
		})
		.sum();

	println!("Sum: {}", sum);
}

