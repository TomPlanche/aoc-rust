use std::collections::HashMap;

///
/// # day_03.rs
/// Code for the day 01 of the Advent of Code challenge year 2023
///

// Imports  ==============================================================================  Imports

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../../data/year_2023/inputs/day_03.txt");

// Functions  =========================================================================== Functions
#[derive(Debug, Clone)]
struct Number {
	value: u16,
	start: (u8, u8),
	end: (u8, u8),
}

///
/// # RandomSymbol
/// A random symbol.
#[derive(Debug)]
struct RandomSymbol {
	position: (u8, u8)
}

///
/// # Gear
/// A gear.
/// Needs adjacent numbers.
#[derive(Debug)]
struct Gear {
	position: (u8, u8),
	adjacent_numbers: Vec<Number>,
}


impl Number {
	///
	/// # is_next_to_symbol
	/// Returns true if the number is next to the given symbol.
	///
	/// ## Arguments
	/// * `self` - The number
	/// * `symbol` - The symbol.
	/// * `verbose` - If true, print the result.
	///
	/// ## Returns
	/// * `bool` - True if the number is next to the given symbol
	fn is_next_to_symbol(
		&self,
		symbol: &RandomSymbol,
		verbose: bool,
	) -> bool {
		let x_min = if self.start.0 <= 1 { 0 } else { self.start.0 - 1 };
		let x_max = self.end.0 + 1;
		let y_min = if self.start.1 < 1 { 0 } else { self.start.1 - 1 };
		let y_max = self.end.1 + 1;

		for x in x_min..=x_max {
			for y in y_min..=y_max {
				if x == symbol.position.0 && y == symbol.position.1 {
					if verbose {
						println!("{} is next to {:?}", self.value, symbol);
					}

					return true;
				}
			}
		}

		false
	}

	///
	/// # is_next_to_symbol
	/// Returns true if the number is next to the given symbol.
	///
	/// ## Arguments
	/// * `self` - The number
	/// * `gear` - The gear.
	/// * `hash_map` - The hash map of gears.
	/// * `verbose` - If true, print the result.
	///
	/// ## Returns
	/// * `bool` - True if the number is next to the given symbol
	fn is_next_to_gear(
		&self,
		gear: &Gear,
		hash_map: &mut HashMap<
			(u8, u8),
			Vec<Number>
		>,
		verbose: bool,
	) -> bool {
		let x_min = if self.start.0 <= 1 { 0 } else { self.start.0 - 1 };
		let x_max = self.end.0 + 1;
		let y_min = if self.start.1 < 1 { 0 } else { self.start.1 - 1 };
		let y_max = self.end.1 + 1;

		for x in x_min..=x_max {
			for y in y_min..=y_max {
				if x == gear.position.0 && y == gear.position.1 {
					if hash_map.contains_key(&gear.position) {
						hash_map.get_mut(&gear.position).unwrap().push(self.clone());
					} else {
						let mut adjacent_numbers = Vec::new();
						adjacent_numbers.push(self.clone());
						hash_map.insert(gear.position, adjacent_numbers);
					}

					if verbose {
						println!("{} is next to {:?}", self.value, gear);
					}

					return true;
				}
			}
		}

		false
	}
}


pub fn response_part_1() {
	println!("Day 03 - Part 1");

	let mut numbers: Vec<Number> = Vec::new();
	let mut symbols: Vec<RandomSymbol> = Vec::new();

	let number_regex = regex::Regex::new(r"\d+").unwrap();
	let symbol_regex = regex::Regex::new(r"[^0-9.]").unwrap();


	// get the input vector of line length
	let lines: Vec<&str> = INPUT
		.lines()
		.collect();

	lines
		.iter()
		.enumerate()
		.for_each(|(index, line)| {
			// regex to parse all the numbers in the line
			for captures in number_regex.captures_iter(line) {
				let number = captures.get(0).unwrap().as_str().parse::<u16>().unwrap();

				let start = captures.get(0).unwrap().start();
				let end = captures.get(0).unwrap().end() - 1;

				let start = (index as u8, start as u8);
				let end = (index as u8, end as u8);

				numbers.push(Number {
					value: number,
					start,
					end,
				});
			}

			// regex to parse all the symbols in the line
			for captures in symbol_regex.captures_iter(line) {
				let position = captures.get(0).unwrap().start();
				let position = (index as u8, position as u8);

				symbols.push( RandomSymbol {
					position,
				});
			}
		});

	let sum = numbers
		.iter()
		.filter(|number| {
			symbols
				.iter()
				.any(|symbol| number.is_next_to_symbol(symbol, false))
		})
		.map(|number| number.value as u32)
		.sum::<u32>();

	println!("Sum: {}", sum);
}

pub fn response_part_2() {
	println!("Day 03 - Part 2");

	let mut numbers: Vec<Number> = Vec::new();
	let mut gears: Vec<Gear> = Vec::new();

	let number_regex = regex::Regex::new(r"\d+").unwrap();
	let gear_regex = regex::Regex::new(r"\*").unwrap();


	// get the input vector of line length
	let lines: Vec<&str> = INPUT
		.lines()
		.collect();

	lines
		.iter()
		.enumerate()
		.for_each(|(index, line)| {
			// regex to parse all the numbers in the line
			for captures in number_regex.captures_iter(line) {
				let number = captures.get(0).unwrap().as_str().parse::<u16>().unwrap();

				let start = captures.get(0).unwrap().start();
				let end = captures.get(0).unwrap().end() - 1;

				let start = (index as u8, start as u8);
				let end = (index as u8, end as u8);

				numbers.push(Number {
					value: number,
					start,
					end,
				});
			}

			// regex to parse all the symbols in the line
			for captures in gear_regex.captures_iter(line) {
				let position = captures.get(0).unwrap().start();
				let position = (index as u8, position as u8);

				// create the mutable gear
				let gear = Gear {
					position,
					adjacent_numbers: Vec::new(),
				};

				// add the mutable gear to the gears vector
				gears.push(gear);
			}
		});


	let mut gears_hm: HashMap<
		(u8, u8),
		Vec<Number>
	> = HashMap::new();

	for number in numbers.iter() {
		for gear in gears.iter() {
			number.is_next_to_gear(gear, &mut gears_hm, false);
		}
	}

	let sum: u32 = gears_hm
		.iter()
		.filter(|(_, adjacent_numbers)| {
			adjacent_numbers.len() == 2 // only the gears with 2 adjacent numbers
		})
		.map(|(_, adjacent_numbers)| {
			adjacent_numbers
				.iter()
				.map(|number| number.value as u32) // get the value of the number
				.reduce(|acc: u32, number: u32| acc * number) // multiply the values
		})
		.flatten() // flatten the Option
		.sum();

	println!("Sum: {}", sum);
}
