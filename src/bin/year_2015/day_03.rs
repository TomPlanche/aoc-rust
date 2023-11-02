///
/// # day_03.rs
/// Code for the day 01 of the Advent of Code challenge year 2015
///

// Imports  ==============================================================================  Imports

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../../data/year_2015/inputs/day_03.txt");
// Functions  =========================================================================== Functions
pub fn response_part_1() {
	println!("Day 03 - Part 1");

	let mut houses = std::collections::HashSet::new();
	let mut x = 0;
	let mut y = 0;
	houses.insert((x, y)); // The first house is always visited

	for c in INPUT.chars() {
		match c {
			'<' => x -= 1,
			'>' => x += 1,
			'^' => y += 1,
			'v' => y -= 1,
			_ => (),
		}
		houses.insert((x, y));
	}

	println!("Houses {:?}", houses);
	println!("Number of houses visited: {}", houses.len());
}

pub fn response_part_2() {
	println!("Day 03 - Part 2");

	let mut houses = std::collections::HashSet::new();
	let mut santa_x = 0;
	let mut santa_y = 0;
	let mut robot_x = 0;
	let mut robot_y = 0;
	houses.insert((santa_x, santa_y)); // The first house is always visited

	for (i, c) in INPUT.chars().enumerate() {
		match c {
			'<' => if i % 2 == 0 { santa_x -= 1 } else { robot_x -= 1 },
			'>' => if i % 2 == 0 { santa_x += 1 } else { robot_x += 1 },
			'^' => if i % 2 == 0 { santa_y += 1 } else { robot_y += 1 },
			'v' => if i % 2 == 0 { santa_y -= 1 } else { robot_y -= 1 },
			_ => (),
		}
		if i % 2 == 0 {
			houses.insert((santa_x, santa_y));
		} else {
			houses.insert((robot_x, robot_y));
		}
	}

	println!("Number of houses visited: {}", houses.len());
}

