///
/// # day_12.rs
/// Code for the day 01 of the Advent of Code challenge year 2022
///
// Imports  ==============================================================================  Imports
use std::{collections::VecDeque, str::FromStr};

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../../data/year_2022/inputs/day_12.txt");

#[derive(Clone, Copy, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

struct HeightMap {
    map: Vec<Vec<u8>>,
    start: Point,
    end: Point,
}

impl HeightMap {
    fn from_str(s: &str) -> Self {
        let mut map = Vec::new();
        let mut start = Point { x: 0, y: 0 };
        let mut end = Point { x: 0, y: 0 };

        for (y, line) in s.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                match c {
                    'S' => {
                        start = Point { x, y };
                        row.push(b'a');
                    }
                    'E' => {
                        end = Point { x, y };
                        row.push(b'z');
                    }
                    _ => row.push(c as u8),
                }
            }
            map.push(row);
        }

        HeightMap { map, start, end }
    }

    ///
    /// # is_valid_move
    ///
    /// Check if the move is valid.
    /// A move is valid if the next point is within the map bounds and the height difference is at most 1.
    ///
    /// ## Arguments
    ///
    /// * `from` - The starting point.
    /// * `to` - The destination point.
    ///
    /// ## Returns
    ///
    /// * `bool` - True if the move is valid, false otherwise.
    fn is_valid_move(&self, from: Point, to: Point) -> bool {
        to.x < self.map[0].len() // Check if the next point is within the map bounds
            && to.y < self.map.len() // Check if the next point is within the map bounds
            && self.map[to.y][to.x] <= self.map[from.y][from.x] + 1 // Check if the height difference is at most 1
    }

    ///
    /// # find_shortest_path
    ///
    /// Find the shortest path from the start to the end point.
    /// It uses a breadth-first search algorithm to find the shortest path.
    ///
    /// ## Returns
    ///
    /// * `Option<usize>` - The number of steps required to reach the end point.
    fn find_shortest_path(&self) -> Option<usize> {
        let mut queue = VecDeque::new(); // Create a queue to store the points to visit
        let mut visited = vec![vec![false; self.map[0].len()]; self.map.len()]; // Create a 2D array to store the visited points

        queue.push_back((self.start, 0)); // Add the start point to the queue
        visited[self.start.y][self.start.x] = true; // Mark the start point as visited

        // Loop until the queue is empty
        while let Some((current, steps)) = queue.pop_front() {
            // Check if the current point is the end point
            if current == self.end {
                return Some(steps);
            }

            // Check the four possible directions
            for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let next = Point {
                    x: (current.x as i32 + dx) as usize,
                    y: (current.y as i32 + dy) as usize,
                }; // Calculate the next point

                // Check if the move is valid and the next point has not been visited
                if self.is_valid_move(current, next) && !visited[next.y][next.x] {
                    visited[next.y][next.x] = true; // Mark the next point as visited
                    queue.push_back((next, steps + 1)); // Add the next point to the queue
                }
            }
        }

        None
    }

    ///
    /// # find_shortest_path_from_any_a
    ///
    /// Find the shortest path from any 'a' elevation point to the end point.
    /// It uses a breadth-first search algorithm to find the shortest path.
    ///
    /// ## Returns
    ///
    /// * `Option<usize>` - The number of steps required to reach the end point.
    fn find_shortest_path_from_any_a(&self) -> Option<usize> {
        let mut queue = VecDeque::new();
        let mut visited = vec![vec![false; self.map[0].len()]; self.map.len()];

        // Instead of starting from the start point, we start from any 'a' elevation point
        for (y, row) in self.map.iter().enumerate() {
            for (x, &height) in row.iter().enumerate() {
                if height == b'a' {
                    queue.push_back((Point { x, y }, 0));
                    visited[y][x] = true;
                }
            }
        }

        // same as `find_shortest_path`
        while let Some((current, steps)) = queue.pop_front() {
            if current == self.end {
                return Some(steps);
            }

            for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let next = Point {
                    x: (current.x as i32 + dx) as usize,
                    y: (current.y as i32 + dy) as usize,
                };

                if self.is_valid_move(current, next) && !visited[next.y][next.x] {
                    visited[next.y][next.x] = true;
                    queue.push_back((next, steps + 1));
                }
            }
        }

        None
    }
}
// Functions  =========================================================================== Functions
pub fn response_part_1() {
    println!("Day 12 - Part 1");

    let height_map = HeightMap::from_str(INPUT);
    if let Some(steps) = height_map.find_shortest_path() {
        println!("Fewest steps required: {}", steps);
    } else {
        println!("No path found");
    }
}

pub fn response_part_2() {
    println!("Day 12 - Part 2");

    let height_map = HeightMap::from_str(INPUT);
    if let Some(steps) = height_map.find_shortest_path_from_any_a() {
        println!("Fewest steps required from any 'a' elevation: {}", steps);
    } else {
        println!("No path found");
    }
}
