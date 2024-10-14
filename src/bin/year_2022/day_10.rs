///
/// # day_10.rs
/// Code for the day 01 of the Advent of Code challenge year 2022
///
// Imports  ==============================================================================  Imports
use std::str::FromStr;

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../../data/year_2022/inputs/day_10.txt");
const CRT_WIDTH: usize = 40;
const CRT_HEIGHT: usize = 6;

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        match parts[0] {
            "noop" => Ok(Instruction::Noop),
            "addx" => Ok(Instruction::Addx(parts[1].parse().map_err(|_| ())?)),
            _ => Err(()),
        }
    }
}
// Functions  =========================================================================== Functions
///
/// # check_signal_strength
///
/// Check the signal strength for a given cycle.
///
/// ## Arguments
///
/// * `cycle` - The current cycle.
/// * `x` - The current x value.
/// * `sum` - The sum of the signal strengths.
fn check_signal_strength(cycle: i32, x: i32, sum: &mut i32) {
    if (cycle - 20) % 40 == 0 && cycle <= 220 {
        *sum += cycle * x;
    }
}

/// # draw_pixel
///
/// Draw a pixel on the CRT screen.
/// The pixel is drawn at the current cycle and x value.
/// The pixel is drawn as a '#' character.
/// The CRT screen is a 6x40 grid.
///
/// ## Arguments
///
/// * `crt` - The CRT screen.
/// * `cycle` - The current cycle.
/// * `x` - The current x value.
fn draw_pixel(crt: &mut [[char; CRT_WIDTH]; CRT_HEIGHT], cycle: i32, x: i32) {
    let row = (cycle / CRT_WIDTH as i32) as usize;
    let col = (cycle % CRT_WIDTH as i32) as usize;

    if (x - 1..=x + 1).contains(&(col as i32)) {
        crt[row][col] = '#';
    }
}

pub fn response_part_1() {
    println!("Day 10 - Part 1");

    let instructions: Vec<Instruction> =
        INPUT.lines().filter_map(|line| line.parse().ok()).collect();

    let mut x = 1;
    let mut cycle = 0;
    let mut signal_strength_sum = 0;

    for instruction in instructions {
        match instruction {
            Instruction::Noop => {
                cycle += 1;
                check_signal_strength(cycle, x, &mut signal_strength_sum);
            }
            Instruction::Addx(v) => {
                for _ in 0..2 {
                    cycle += 1;
                    check_signal_strength(cycle, x, &mut signal_strength_sum);
                }
                x += v;
            }
        }
    }

    println!("Sum of signal strengths: {}", signal_strength_sum);
}

pub fn response_part_2() {
    println!("Day 10 - Part 2");

    let instructions: Vec<Instruction> =
        INPUT.lines().filter_map(|line| line.parse().ok()).collect();

    let mut x = 1;
    let mut cycle = 0;
    let mut crt = [['.'; CRT_WIDTH]; CRT_HEIGHT];

    for instruction in instructions {
        match instruction {
            Instruction::Noop => {
                draw_pixel(&mut crt, cycle, x);
                cycle += 1;
            }
            Instruction::Addx(v) => {
                for _ in 0..2 {
                    draw_pixel(&mut crt, cycle, x);
                    cycle += 1;
                }
                x += v;
            }
        }
    }

    println!("CRT Output:");
    for row in &crt {
        println!("{}", row.iter().collect::<String>());
    }
}
