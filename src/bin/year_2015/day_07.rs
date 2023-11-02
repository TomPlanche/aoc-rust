///
/// # day_07.rs
/// Code for the day 01 of the Advent of Code challenge year 2015
///

// Imports  ==============================================================================  Imports
use std::collections::HashMap;

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../../data/year_2015/inputs/day_07.txt");

// Functions  =========================================================================== Functions
///
/// # parse_line
/// Main function for parsing the input.
///
/// A line can be of the following types:
/// * `123 -> x` - The signal 123 is provided to wire x.
/// * `x AND y -> z` - Bitwise AND of wire x and wire y is provided to wire z.
/// * `NOT x -> z` - The bitwise complement of the signal from wire x is provided to wire z.
///
/// ## Arguments
/// * `wires` - The HashMap containing the wires
/// * `line` - The line to parse
fn parse_line(wires: &mut HashMap<String, u16>, line: &str) {
	let regex_rule_1 = regex::Regex::new(r"^(?P<signal>\d+) -> (?P<wire>\w+)$").unwrap();
	let regex_rule_2 = regex::Regex::new(r"^(?P<wire_1>\w+) (?P<operator>AND|OR|LSHIFT|RSHIFT) (?P<shift_val>\w+) -> (?P<wire>\w+)$").unwrap();
	let regex_rule_3 = regex::Regex::new(r"^NOT (?P<wire_1>\w+) -> (?P<wire>\w+)$").unwrap();

	if regex_rule_1.is_match(line) {
		let captures = regex_rule_1.captures(line).unwrap();
		let signal = captures.name("signal").unwrap().as_str().parse::<u16>().unwrap();
		let wire = captures.name("wire").unwrap().as_str();

		println!("Rule 1: {} -> {}", signal, wire);
	} else if regex_rule_2.is_match(line) {
		let captures = regex_rule_2.captures(line).unwrap();
		let wire_1 = captures.name("wire_1").unwrap().as_str();
		let operator = captures.name("operator").unwrap().as_str();
		let shift_val = captures.name("shift_val").unwrap().as_str();
		let wire = captures.name("wire").unwrap().as_str();

		println!("Rule 2: {} {} {} -> {}", wire_1, operator, shift_val, wire);
	} else if regex_rule_3.is_match(line) {
		let captures = regex_rule_3.captures(line).unwrap();
		let wire_1 = captures.name("wire_1").unwrap().as_str();
		let wire = captures.name("wire").unwrap().as_str();

		println!("Rule 3: NOT {} -> {}", wire_1, wire);
	} else {
		panic!("Unknown line: {}", line);
	}
}

pub fn response_part_1() {
	println!("Day 07 - Part 1");
	let mut wires: HashMap<String, u16> = HashMap::new();
	for line in INPUT.lines() {
		parse_line(&mut wires, line);
		break;
	}
}

pub fn response_part_2() {
	println!("Day 07 - Part 2");
}

