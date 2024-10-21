use std::collections::{HashMap, HashSet};
use std::fmt;
///
/// # day_16.rs
/// Code for the day 16 of the Advent of Code challenge year 2022
///
// Imports  ==============================================================================  Imports
use std::str::FromStr;

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../../data/year_2022/inputs/day_16.txt");

/// Represents a valve in the cave system
#[derive(Debug, Clone)]
struct Valve {
    name: String,
    flow_rate: u32,
    tunnels: Vec<String>,
    is_open: bool,
}

struct ValveSystem {
    valves: HashMap<String, Valve>,
    current_valve: String,
    time_remaining: u32,
    pressure_released: u32,
}

impl FromStr for Valve {
    type Err = fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(';').collect();
        if parts.len() != 2 {
            return Err(fmt::Error);
        }

        let valve_info: Vec<&str> = parts[0].split_whitespace().collect();
        if valve_info.len() < 5 {
            return Err(fmt::Error);
        }

        let name = valve_info[1].to_string();
        let flow_rate = valve_info[4]
            .trim_start_matches("rate=")
            .parse::<u32>()
            .map_err(|_| fmt::Error)?;

        let tunnels: Vec<String> = parts[1]
            .trim()
            .trim_start_matches("tunnels lead to valves ")
            .trim_start_matches("tunnel leads to valve ")
            .split(", ")
            .map(|s| s.to_string())
            .collect();

        Ok(Valve {
            name,
            flow_rate,
            tunnels,
            is_open: false,
        })
    }
}

impl ValveSystem {
    fn new(valves: Vec<Valve>) -> Self {
        let valves_map = valves.into_iter().map(|v| (v.name.clone(), v)).collect();
        ValveSystem {
            valves: valves_map,
            current_valve: "AA".to_string(),
            time_remaining: 30,
            pressure_released: 0,
        }
    }

    /// # find_optimal_path
    ///
    /// Find the optimal path to release the most pressure
    ///
    /// ## Returns
    /// * u32 - The maximum pressure that can be released in 30 minutes
    fn find_optimal_path(&self) -> u32 {
        // Filter out valves with zero flow rate as they don't contribute to pressure release
        let valuable_valves: Vec<&String> = self
            .valves
            .iter()
            .filter(|(_, v)| v.flow_rate > 0)
            .map(|(name, _)| name)
            .collect();

        // Start the DFS from valve "AA" with 30 minutes and no pressure released
        self.dfs_optimal_path("AA", 30, &valuable_valves, 0)
    }

    /// # dfs_optimal_path
    ///
    /// Perform a depth-first search to find the optimal path for releasing pressure
    ///
    /// ## Arguments
    ///
    /// * `current` - The name of the current valve
    /// * `time_left` - Remaining time in minutes
    /// * `unopened` - List of unopened valves with non-zero flow rates
    /// * `pressure` - Total pressure released so far
    ///
    /// ## Returns
    /// * u32 - The maximum pressure that can be released following this path
    fn dfs_optimal_path(
        &self,
        current: &str,
        time_left: i32,
        unopened: &[&String],
        pressure: u32,
    ) -> u32 {
        // Base case: if we're out of time or all valves are opened, return current pressure
        if time_left <= 0 || unopened.is_empty() {
            return pressure;
        }

        let mut max_pressure = pressure;

        // Try opening each unopened valve
        for (i, &valve) in unopened.iter().enumerate() {
            // Calculate time to reach and open the valve
            let distance = self.shortest_distance(current, valve);
            let new_time_left = time_left - distance - 1; // -1 for opening the valve

            // If we don't have time to reach and open the valve, skip it
            if new_time_left <= 0 {
                continue;
            }

            // Calculate new pressure released if we open this valve
            let new_pressure = pressure + (new_time_left as u32 * self.valves[valve].flow_rate);

            // Create a new list of unopened valves, excluding the current one
            let new_unopened: Vec<&String> = unopened
                .iter()
                .enumerate()
                .filter(|&(j, _)| j != i)
                .map(|(_, &v)| v)
                .collect();

            // Recursively explore this path
            let result = self.dfs_optimal_path(valve, new_time_left, &new_unopened, new_pressure);

            // Update max_pressure if this path yields a better resul
            max_pressure = max_pressure.max(result);
        }

        max_pressure
    }

    /// # shortest_distance
    ///
    /// Calculate the shortest distance between two valves using Breadth-First Search (BFS)
    ///
    /// ## Arguments
    ///
    /// * `start` - The name of the starting valve
    /// * `end` - The name of the destination valve
    ///
    /// ## Returns
    /// * i32 - The shortest distance (in steps) between the start and end valves
    fn shortest_distance(&self, start: &str, end: &str) -> i32 {
        let mut queue = std::collections::VecDeque::new();
        let mut visited = HashSet::new();

        // Initialize BFS with the starting valve
        queue.push_back((start, 0));
        visited.insert(start);

        while let Some((current, distance)) = queue.pop_front() {
            // If we've reached the destination, return the distance
            if current == end {
                return distance;
            }

            // Explore all neighboring valves
            for neighbor in &self.valves[current].tunnels {
                if !visited.contains(neighbor.as_str()) {
                    visited.insert(neighbor);
                    queue.push_back((neighbor, distance + 1));
                }
            }
        }

        i32::MAX // This should never happen if the graph is connected
    }

    // Other helper methods as needed
}
// Functions  =========================================================================== Functions
pub fn response_part_1() {
    println!("Day 16 - Part 1");

    let valves: Vec<Valve> = INPUT.lines().map(|line| line.parse().unwrap()).collect();
    let system = ValveSystem::new(valves);
    let max_pressure = system.find_optimal_path();

    println!("Maximum pressure that can be released: {}", max_pressure);
}

pub fn response_part_2() {
    println!("Day 16 - Part 2");
}

// Tests ==================================================================================== Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valve_from_str() {
        let valve: Valve = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB"
            .parse()
            .unwrap();

        assert_eq!(valve.name, "AA");
        assert_eq!(valve.flow_rate, 0);
        assert_eq!(
            valve.tunnels,
            vec!["DD".to_string(), "II".to_string(), "BB".to_string()]
        );
    }
}
