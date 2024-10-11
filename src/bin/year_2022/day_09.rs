///
/// # day_09.rs
/// Code for the day 01 of the Advent of Code challenge year 2022
///
// Imports  ==============================================================================  Imports
use std::collections::HashSet;

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../../data/year_2022/inputs/day_09.txt");

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn move_direction(&mut self, direction: char) {
        match direction {
            'U' => self.y += 1,
            'D' => self.y -= 1,
            'L' => self.x -= 1,
            'R' => self.x += 1,
            _ => panic!("Invalid direction"),
        }
    }

    ///
    /// # follow
    ///
    /// Move the position to follow another position.
    ///
    /// ## Arguments
    ///
    /// * `other` - The other position to follow.
    fn follow(&mut self, other: &Position) {
        let dx = other.x - self.x;
        let dy = other.y - self.y;

        if dx.abs() > 1 || dy.abs() > 1 {
            self.x += dx.signum(); // signum = 1 if positive, -1 if negative, 0 if zero
            self.y += dy.signum();
        }
    }
}

// Functions  =========================================================================== Functions
///
/// # simulate_rope
///
/// This function simulates the movement of a rope with a specified number of knots.
/// It processes the input instructions, moves the head of the rope accordingly,
/// and updates the positions of all following knots. It keeps track of all unique
/// positions visited by the tail (last knot) of the rope.
///
/// ## Arguments
///
/// * `knot_count` - The number of knots in the rope.
///
/// ## Returns
///
/// The number of unique positions visited by the tail of the rope.
fn simulate_rope(knot_count: usize) -> usize {
    // initialize the rope with the head at the origin
    // and the tail at the origin
    let mut rope = vec![Position::default(); knot_count];
    let mut tail_positions = HashSet::new();
    tail_positions.insert(Position::default());

    // process the input instructions
    for line in INPUT.lines() {
        let (direction, steps) = line.split_once(' ').unwrap();
        let direction = direction.chars().next().unwrap();
        let steps = steps.parse::<usize>().unwrap();

        // move the head and update the tail positions
        for _ in 0..steps {
            // move the head
            rope[0].move_direction(direction);

            // follow the head
            for i in 1..knot_count {
                let previous = rope[i - 1];

                // move the current knot to follow the previous one
                rope[i].follow(&previous);
            }

            // add the new tail position to the set
            tail_positions.insert(rope[knot_count - 1]);
        }
    }

    // return the number of unique tail positions
    tail_positions.len()
}

pub fn response_part_1() {
    println!("Day 09 - Part 1");

    let result = simulate_rope(2);
    println!(
        "The tail of the rope visited {} positions at least once.",
        result
    );
}

pub fn response_part_2() {
    println!("Day 09 - Part 2");

    let result = simulate_rope(10);

    println!(
        "The tail of the rope visited {} positions at least once.",
        result
    );
}
