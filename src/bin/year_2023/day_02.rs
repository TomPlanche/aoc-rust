///
/// # day_02.rs
/// Code for the day 01 of the Advent of Code challenge year 2023
///

// Imports  ==============================================================================  Imports

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../../data/year_2023/inputs/day_02.txt");

// Functions  =========================================================================== Functions

pub fn response_part_1() {
	println!("Day 02 - Part 1");

	// regex to parse the line
	let line_regex = regex::Regex::new(r"Game (?P<game_id>\d+):(?P<sets>.*)").unwrap();
	// regex to parse the set
	let set_regex = regex::Regex::new(r"(?P<digit>\d+) (?P<color>blue|red|green)").unwrap();

	let red_limit = 12u8;
	let green_limit = 13u8;
	let blue_limit = 14u8;

	let sum: u32 = INPUT
		.lines()
		.map(|line| {
			let captures = line_regex.captures(line).unwrap();
			let game_id = captures.name("game_id").unwrap().as_str().parse::<u8>().unwrap();

			let invalid_sets: u8 = captures.name("sets").unwrap().as_str()
				.split("; ")
				.map(|set| {
					let set = set.trim();

					let mut blue_cpt = 0u8;
					let mut green_cpt = 0u8;
					let mut red_cpt = 0u8;

					// map through the set splitted by ', '
					set.split(", ")
						.map(|cube_infos| {
							let cube_infos = cube_infos.trim();

							let captures = set_regex.captures(cube_infos).unwrap();
							let digit = captures.name("digit").unwrap().as_str().parse::<u8>().unwrap();
							let color = captures.name("color").unwrap().as_str();

							match color {
								"red" => red_cpt += digit,
								"green" => green_cpt += digit,
								"blue" => blue_cpt += digit,
								_ => panic!("Unknown color: {}", color),
							}
						})
						.count();


					if red_cpt > red_limit
						|| green_cpt > green_limit
						|| blue_cpt > blue_limit {
						1
					} else {
						0
					}

				}).sum();



			if invalid_sets > 0 {
				0
			} else {
				game_id as u32
			}
		})
		.sum();

	println!("Sum: {}", sum);
}

pub fn response_part_2() {
	println!("Day 02 - Part 2");

	let line_regex = regex::Regex::new(r"Game (?P<game_id>\d+):(?P<sets>.*)").unwrap();
	let set_regex = regex::Regex::new(r"(?P<digit>\d+) (?P<color>blue|red|green)").unwrap();

	let sum: u32 = INPUT
		.lines()
		.map(|line| {
			let captures = line_regex.captures(line).unwrap();

			let mut blue_max = 0u16;
			let mut green_max = 0u16;
			let mut red_max = 0u16;

			let _: u8 = captures.name("sets").unwrap().as_str()
				.split("; ")
				.map(|set| {
					// remove the leading and trailing spaces
					let set = set.trim();
					// map through the set splitted by ', '
					let _: u8 = set.split(", ")
						.map(|cube_infos| {
							let cube_infos = cube_infos.trim();

							let captures = set_regex.captures(cube_infos).unwrap();
							let digit = captures.name("digit").unwrap().as_str().parse::<u16>().unwrap();
							let color = captures.name("color").unwrap().as_str();

							match color {
								"red" => red_max = if red_max > digit { red_max } else { digit },
								"green" => green_max = if green_max > digit { green_max } else { digit },
								"blue" => blue_max = if blue_max > digit { blue_max } else { digit },
								_ => panic!("Unknown color: {}", color),
							}

							0
						}).sum();


					0
				}).sum();
			(red_max * green_max * blue_max) as u32
		}).sum();

	println!("Sum: {}", sum);
}

