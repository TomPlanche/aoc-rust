///
/// # src/main.rs
/// Contains the main function for the program.
///

// Imports  ==============================================================================  Imports
#[path = "./bin/year_2015/day_06.rs"]
mod day;

use day::response_part_1;
use day::response_part_2;
// Variables  =========================================================================== Variables

// Functions  =========================================================================== Functions

// Main  ====================================================================================  Main
fn main() {
	response_part_1();
	println!();
	response_part_2();
}
