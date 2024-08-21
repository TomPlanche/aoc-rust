///
/// # day_02.rs
/// Code for the day 01 of the Advent of Code challenge year 2022
///

// Imports  ==============================================================================  Imports

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../../data/year_2022/inputs/day_02.txt");

// Functions  =========================================================================== Functions
fn score_for_round(opponent_moove: char, player_moove: char) -> u32 {
    match (opponent_moove, player_moove) {
        ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6,
        ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3,
        ('A', 'Z') | ('B', 'X') | ('C', 'Y') => 0,
        _ => panic!("Invalid choice made"),
    }
}

fn score_for_player_moove(player_moove: char) -> u32 {
    match player_moove {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => panic!("Invalid choice made"),
    }
}

pub fn response_part_1() {
    println!("Day 02 - Part 1");

    let line_regex =
        regex::Regex::new(r"(?P<opponent_moove>[A-Z]) (?P<player_moove>[A-Z])").unwrap();

    let total = INPUT
        .lines()
        .map(|line| {
            let captures = line_regex.captures(line).unwrap();
            let opponent_moove = captures
                .name("opponent_moove")
                .unwrap()
                .as_str()
                .chars()
                .next()
                .unwrap();
            let player_moove = captures
                .name("player_moove")
                .unwrap()
                .as_str()
                .chars()
                .next()
                .unwrap();

            score_for_round(opponent_moove, player_moove) + score_for_player_moove(player_moove)
        })
        .sum::<u32>();

    println!("Total: {}", total);
}

pub fn response_part_2() {
    println!("Day 02 - Part 2");

    fn choose_best_moove(opponent_moove: char, what_to_do: char) -> char {
        match (what_to_do, opponent_moove) {
            ('X', 'A') => 'Z',
            ('X', 'B') => 'X',
            ('X', 'C') => 'Y',
            ('Y', 'A') => 'X',
            ('Y', 'B') => 'Y',
            ('Y', 'C') => 'Z',
            ('Z', 'A') => 'Y',
            ('Z', 'B') => 'Z',
            ('Z', 'C') => 'X',
            _ => panic!(
                "Invalid choice p;rovided, what to do: {} - {}",
                opponent_moove, what_to_do
            ),
        }
    }

    let line_regex = regex::Regex::new(r"(?P<opponent_moove>[A-Z]) (?P<what_to_do>[A-Z])").unwrap();

    let total = INPUT
        .lines()
        .map(|line| {
            let captures = line_regex.captures(line).unwrap();
            let opponent_moove = captures
                .name("opponent_moove")
                .unwrap()
                .as_str()
                .chars()
                .next()
                .unwrap();
            let what_to_do = captures
                .name("what_to_do")
                .unwrap()
                .as_str()
                .chars()
                .next()
                .unwrap();

            let player_moove = choose_best_moove(opponent_moove, what_to_do);

            score_for_round(opponent_moove, player_moove) + score_for_player_moove(player_moove)
        })
        .sum::<u32>();

    println!("Total: {}", total);
}
