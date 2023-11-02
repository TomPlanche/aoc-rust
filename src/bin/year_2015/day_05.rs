///
/// # day_05.rs
/// Code for the day 01 of the Advent of Code challenge year 2015
///

// Imports  ==============================================================================  Imports

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../../data/year_2015/inputs/day_05.txt");

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const FORBIDDEN: [&str; 4] = ["ab", "cd", "pq", "xy"];
// Functions  =========================================================================== Functions
fn string_contains_three_vowels(word: &str) -> bool {
	let mut cpt: usize = 0;
	for vowel in VOWELS.iter() {
		cpt += word.matches(*vowel).count();

		if cpt >= 3 {
			return true;
		}
	}

	false
}

fn string_contains_double_letter(word: &str) -> bool {
	word.as_bytes().windows(2).any(|w| w[0] == w[1])
}

fn string_contains_forbidden(word: &str) -> bool {
	FORBIDDEN.iter().any(|f| word.contains(f))
}

fn evaluate_string(word: &str) -> bool {
	string_contains_three_vowels(word) && string_contains_double_letter(word) && !string_contains_forbidden(word)
}

pub fn response_part_1() {
	println!("Day 05 - Part 1");

	let cpt = INPUT.lines().map(|l| evaluate_string(l)).filter(|b| *b).count();

	print!("There are {} nice strings.", cpt);

}

// Part 2 functions
fn string_contains_double_pair(word: &str) -> bool {
	word.as_bytes().windows(2).enumerate().any(|(i, w)| {
		let pair = &word[i + 2..];
		pair.as_bytes().windows(2).any(|p| p == w)
	})
}

fn string_contains_sandwich(word: &str) -> bool {
	word.as_bytes().windows(3).any(|w| w[0] == w[2])
}

pub fn response_part_2() {
	println!("Day 05 - Part 2");

	let cpt = INPUT.lines()
		.map(|l| string_contains_double_pair(l) && string_contains_sandwich(l))
		.filter(|b| *b).count();

	print!("There are {} nice strings.", cpt);
}

// Tests ==================================================================================== Tests
#[test]
fn test_word_contains_three_vowels() {
	assert_eq!(string_contains_three_vowels("aei"), true);
	assert_eq!(string_contains_three_vowels("xazegov"), true);
	assert_eq!(string_contains_three_vowels("aeiouaeiouaeiou"), true);
	assert_eq!(string_contains_three_vowels("dvszwmarrgswjxmb"), false);
}

#[test]
fn test_word_contains_double_letter() {
	assert_eq!(string_contains_double_letter("xx"), true);
	assert_eq!(string_contains_double_letter("abcdde"), true);
	assert_eq!(string_contains_double_letter("aabbccdd"), true);
	assert_eq!(string_contains_double_letter("jchzalrnumimnmhp"), false);
}
