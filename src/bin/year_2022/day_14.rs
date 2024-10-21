// Imports  ==============================================================================  Imports
use crate::point::Point;

use std::{collections::HashSet, fmt, str::FromStr};

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../../data/year_2022/inputs/day_14.txt");
const SAND_SOURCE: MyPoint = MyPoint { x: 500, y: 0 };

type MyPoint = Point<i32>;

impl FromStr for MyPoint {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap();
        Ok(MyPoint {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

struct Cave {
    rocks: HashSet<MyPoint>,
    sand: HashSet<MyPoint>,
    abyss_y: i32,
    floor_y: i32,
}

impl FromStr for Cave {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rocks = HashSet::new();
        for line in s.lines() {
            let point: Vec<MyPoint> = line.split(" -> ").map(|s| s.parse().unwrap()).collect();

            for window in point.windows(2) {
                let [start, end] = window else { continue };
                for x in start.x.min(end.x)..=start.x.max(end.x) {
                    for y in start.y.min(end.y)..=start.y.max(end.y) {
                        rocks.insert(MyPoint { x, y });
                    }
                }
            }
        }

        let abyss_y = rocks.iter().map(|p| p.y).max().unwrap() + 1;

        Ok(Cave {
            rocks,
            sand: HashSet::new(),
            abyss_y,
            floor_y: abyss_y + 1, // The floor is one unit below the abyss
        })
    }
}

impl Cave {
    /// # simulate_sand
    ///
    /// Simulate the fall of a single unit of sand
    ///
    /// ## Arguments
    ///
    /// * `part2` - Boolean flag to indicate if we're solving part 2 of the puzzle
    ///
    /// ## Returns
    ///
    /// * `bool` - True if the sand came to rest, false if it fell into the abyss or source is blocked
    fn simulate_sand(&mut self, part2: bool) -> bool {
        let mut sand = SAND_SOURCE;
        if self.is_occupied(sand) {
            return false; // Source is blocked
        }
        loop {
            if sand.y + 1 == self.floor_y && !part2 {
                return false; // Sand falls into the abyss in part 1
            }
            if !self.is_occupied(MyPoint {
                x: sand.x,
                y: sand.y + 1,
            }) {
                sand.y += 1;
            } else if !self.is_occupied(MyPoint {
                x: sand.x - 1,
                y: sand.y + 1,
            }) {
                sand.x -= 1;
                sand.y += 1;
            } else if !self.is_occupied(MyPoint {
                x: sand.x + 1,
                y: sand.y + 1,
            }) {
                sand.x += 1;
                sand.y += 1;
            } else {
                self.sand.insert(sand);
                return true;
            }
        }
    }

    ///  # is_occupied
    ///
    /// Check if a given point is occupied by rock, sand, or floor
    ///
    /// ## Arguments
    ///
    /// * `p` - The point to check
    ///
    /// ## Returns
    ///
    /// * `bool` - True if the point is occupied, false otherwise
    fn is_occupied(&self, p: MyPoint) -> bool {
        self.rocks.contains(&p) || self.sand.contains(&p) || p.y == self.floor_y
    }

    ///
    /// # simulate_sand_step
    ///
    /// Simulate a single step of sand falling for visualization
    ///
    /// ## Returns
    ///
    /// * `bool` - True if the sand came to rest, false if it fell into the abyss
    fn simulate_sand_step(&mut self) -> bool {
        let mut sand = SAND_SOURCE;

        loop {
            if sand.y >= self.abyss_y {
                return false; // Sand falls into the abyss
            }
            if !self.is_occupied(MyPoint {
                x: sand.x,
                y: sand.y + 1,
            }) {
                sand.y += 1;
            } else if !self.is_occupied(MyPoint {
                x: sand.x - 1,
                y: sand.y + 1,
            }) {
                sand.x -= 1;
                sand.y += 1;
            } else if !self.is_occupied(MyPoint {
                x: sand.x + 1,
                y: sand.y + 1,
            }) {
                sand.x += 1;
                sand.y += 1;
            } else {
                self.sand.insert(sand);
                return true; // Sand comes to rest
            }
            // Clear console and print the current state
            print!("\x1B[2J\x1B[1;1H");
            println!("{:?}", self);
            std::thread::sleep(std::time::Duration::from_millis(2)); // Adjust speed as needed
        }
    }

    ///
    /// # animate_sand_fall
    ///
    /// Animate the sand falling process
    ///
    /// # Returns
    ///
    /// * `usize` - The number of sand units that came to rest
    #[allow(dead_code)]
    fn animate_sand_fall(&mut self) -> usize {
        let mut count = 0;
        while self.simulate_sand_step() {
            count += 1;
        }
        count
    }

    /// # count_settled_sand
    ///
    /// ## Arguments
    ///
    /// * `part2` - Boolean flag to indicate if we're solving part 2 of the puzzle
    ///
    /// ## Returns
    ///
    /// * `usize` - The number of sand units that came to rest
    fn count_settled_sand(&mut self, part2: bool) -> usize {
        let mut count = 0;
        while self.simulate_sand(part2) {
            count += 1;
        }
        count
    }
}

// Implement Debug for Cave to visualize the cave system
impl fmt::Debug for Cave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let min_x = self
            .rocks
            .iter()
            .chain(self.sand.iter())
            .map(|p| p.x)
            .min()
            .unwrap_or(0)
            .min(SAND_SOURCE.x)
            - 1;
        let max_x = self
            .rocks
            .iter()
            .chain(self.sand.iter())
            .map(|p| p.x)
            .max()
            .unwrap_or(0)
            .max(SAND_SOURCE.x)
            + 1;

        let min_y = 0;
        let max_y = self.floor_y;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let point = MyPoint { x, y };
                if point == SAND_SOURCE {
                    write!(f, "+")?;
                } else if y == self.floor_y {
                    write!(f, "#")?;
                } else if self.rocks.contains(&point) {
                    write!(f, "#")?;
                } else if self.sand.contains(&point) {
                    write!(f, "o")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// Functions  =========================================================================== Functions
pub fn response_part_1() {
    let mut cave: Cave = INPUT.parse().unwrap();

    // let settled_sand = cave.animate_sand_fall();
    let settled_sand = cave.count_settled_sand(false);

    println!("Day 14 - Part 1: {}", settled_sand);
}

pub fn response_part_2() {
    let mut cave: Cave = INPUT.parse().unwrap();
    let settled_sand = cave.count_settled_sand(true);

    println!("{:?}", cave);

    println!("Day 14 - Part 2: {}", settled_sand);
}
