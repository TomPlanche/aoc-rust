///
/// # day_06.rs
/// Code for the day 01 of the Advent of Code challenge year 2015
///

// Imports  ==============================================================================  Imports
use std::cmp::{max, min};
use std::fs::File;
// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../../data/year_2015/inputs/day_06.txt");
const ARRAY_SIZE: u64 = 1000;

trait LightBehaviour {
	fn toggle(&mut self);
	fn turn_on(&mut self);
	fn turn_off(&mut self);
}
// Functions  =========================================================================== Functions
impl LightBehaviour for bool {
	fn toggle(&mut self) {
		*self = !*self;
	}

	fn turn_on(&mut self) {
		*self = true;
	}

	fn turn_off(&mut self) {
		*self = false;
	}
}

impl LightBehaviour for u64 {
	fn toggle(&mut self) {
		*self += 2;
	}

	fn turn_on(&mut self) {
		*self += 1;
	}

	fn turn_off(&mut self) {
		if *self > 0 {
			*self -= 1;
		}
	}
}

///
/// # parse_instruction
/// Parses the given instruction.
/// The instruction should be in the following format:
/// - turn on 0,0 through 999,999
/// - toggle 0,0 through 999,0
/// - turn off 499,499 through 500,500
///
/// ## Arguments
/// * `lights` - The vector of lights
/// * `instruction` - The instruction to parse
///
/// ## Returns
/// * `()` - Nothing
fn parse_instruction<T: LightBehaviour>(
	lights: &mut Vec<Vec<T>>,
	instruction: &str
) {
	let regex = regex::Regex::new(
		r"^(?P<action>turn on|toggle|turn off) (?P<x1>\d+),(?P<y1>\d+) through (?P<x2>\d+),(?P<y2>\d+)$")
		.unwrap();

	let captures = regex.captures(instruction).unwrap();

	let action = captures.name("action").unwrap().as_str();
	let x1 = captures.name("x1").unwrap().as_str().parse::<u64>().unwrap();
	let y1 = captures.name("y1").unwrap().as_str().parse::<u64>().unwrap();
	let x2 = captures.name("x2").unwrap().as_str().parse::<u64>().unwrap();
	let y2 = captures.name("y2").unwrap().as_str().parse::<u64>().unwrap();

	for x in min(x1, x2)..=max(x1, x2) {
		for y in min(y1, y2)..=max(y1, y2) {
			match action {
				"turn on" => lights[x as usize][y as usize].turn_on(),
				"toggle" => lights[x as usize][y as usize].toggle(),
				"turn off" => lights[x as usize][y as usize].turn_off(),
				_ => panic!("Unknown action: {}", action),
			}
		}
	}
}

pub fn response_part_1() {
	println!("Day 06 - Part 1");

	let mut lights_vector = vec![
		vec![false; ARRAY_SIZE as usize];
		ARRAY_SIZE as usize
	];

	for instruction in INPUT.lines() {
		parse_instruction(&mut lights_vector, instruction);
	}

	let lights_on = lights_vector.iter().flatten().filter(|&light| *light).count();

	println!("There are {} lights on.", lights_on);
}

pub fn response_part_2() {
	println!("Day 06 - Part 2");

	let mut lights_vector = vec![
		vec![0u64; ARRAY_SIZE as usize];
		ARRAY_SIZE as usize
	];

	for instruction in INPUT.lines() {
		parse_instruction(&mut lights_vector, instruction);
	}

	let lights_on = lights_vector.iter().flatten().sum::<u64>();

	println!("The total brightness is {}.", lights_on);


}

