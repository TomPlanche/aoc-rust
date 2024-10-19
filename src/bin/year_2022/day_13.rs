///
/// # day_13.rs
/// Code for the day 01 of the Advent of Code challenge year 2022
///
// Imports  ==============================================================================  Imports
use std::cmp::Ordering;
use std::str::FromStr;

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../../data/year_2022/inputs/day_13.txt");

///
/// # PacketData
/// Represents the data structure of a packet, which can be either an integer or a list of PacketData
#[derive(Debug, PartialEq, Eq, Clone)]
enum PacketData {
    Integer(i32),
    List(Vec<PacketData>),
}

/// Wrapper struct for PacketData
#[derive(Debug, Clone)]
struct Packet(PacketData);

/// Custom error type for packet parsing
#[derive(Debug)]
struct ParsePacketError;

impl FromStr for Packet {
    type Err = ParsePacketError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ///
        /// # parse_list
        ///
        /// Helper function to parse a list of PacketData
        /// This function is recursive to handle nested lists
        ///
        /// # Arguments
        /// * `chars` - A mutable reference to a peekable iterator of characters
        ///
        /// # Returns
        /// * `Result<Vec<PacketData>, ParsePacketError>` - The parsed list or an error
        fn parse_list(
            chars: &mut std::iter::Peekable<std::str::Chars>,
        ) -> Result<Vec<PacketData>, ParsePacketError> {
            let mut list = Vec::new();
            while let Some(&c) = chars.peek() {
                match c {
                    '[' => {
                        chars.next();
                        list.push(PacketData::List(parse_list(chars)?));
                    }
                    ']' => {
                        chars.next();
                        return Ok(list);
                    }
                    ',' => {
                        chars.next();
                    }
                    _ => {
                        // Parse integer
                        let mut num = String::new();
                        while let Some(&c) = chars.peek() {
                            if c.is_ascii_digit() {
                                num.push(chars.next().unwrap());
                            } else {
                                break;
                            }
                        }
                        list.push(PacketData::Integer(
                            num.parse().map_err(|_| ParsePacketError)?,
                        ));
                    }
                }
            }
            Err(ParsePacketError)
        }

        let mut chars = s.chars().peekable();
        if chars.next() != Some('[') {
            return Err(ParsePacketError);
        }

        let data = PacketData::List(parse_list(&mut chars)?);
        if chars.next().is_some() {
            return Err(ParsePacketError);
        }

        Ok(Packet(data))
    }
}

/// Implement comparison traits for Packet
/// This allows packets to be compared and sorted
impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.compare(other) == Ordering::Equal
    }
}

impl Eq for Packet {}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.compare(other)
    }
}

impl Packet {
    ///
    /// # compare
    ///
    /// Compare two packets according to the problem's rules
    /// This method implements the custom comparison logic described in the puzzle
    ///
    /// # Arguments
    /// * `other` - The other Packet to compare with
    ///
    /// # Returns
    /// * `Ordering` - Less, Equal, or Greater
    fn compare(&self, other: &Self) -> Ordering {
        ///
        /// # compare_data
        ///
        /// Helper function to compare two PacketData items
        /// This function is recursive to handle nested lists
        ///
        /// # Arguments
        /// * `left` - The left PacketData to compare
        /// * `right` - The right PacketData to compare
        ///
        /// # Returns
        /// * `Ordering` - Less, Equal, or Greater
        fn compare_data(left: &PacketData, right: &PacketData) -> Ordering {
            match (left, right) {
                (PacketData::Integer(l), PacketData::Integer(r)) => l.cmp(r),
                (PacketData::List(l), PacketData::List(r)) => {
                    for (l_item, r_item) in l.iter().zip(r.iter()) {
                        match compare_data(l_item, r_item) {
                            Ordering::Equal => continue,
                            other => return other,
                        }
                    }
                    l.len().cmp(&r.len())
                }
                (PacketData::Integer(_), PacketData::List(_)) => {
                    compare_data(&PacketData::List(vec![left.clone()]), right)
                }
                (PacketData::List(_), PacketData::Integer(_)) => {
                    compare_data(left, &PacketData::List(vec![right.clone()]))
                }
            }
        }

        compare_data(&self.0, &other.0)
    }
}

// Functions  =========================================================================== Functions
pub fn response_part_1() {
    // Parse input into pairs of packets
    let pairs: Vec<_> = INPUT
        .split("\n\n")
        .map(|pair| {
            let mut lines = pair.lines();
            (
                lines.next().unwrap().parse::<Packet>().unwrap(),
                lines.next().unwrap().parse::<Packet>().unwrap(),
            )
        })
        .collect();

    // Sum indices of pairs in the right order
    // We add 1 to the index because the puzzle uses 1-based indexing
    let sum: usize = pairs
        .iter()
        .enumerate()
        .filter(|(_, (left, right))| left.compare(right) == Ordering::Less)
        .map(|(i, _)| i + 1)
        .sum();

    println!("Day 13 - Part 1: {}", sum);
}

pub fn response_part_2() {
    let mut packets: Vec<Packet> = INPUT
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect();

    // Add divider packets
    // These are special packets that we need to track in the sorted list
    let divider1: Packet = "[[2]]".parse().unwrap();
    let divider2: Packet = "[[6]]".parse().unwrap();
    packets.push(divider1.clone());
    packets.push(divider2.clone());

    // Sort packets
    packets.sort();

    // Find indices of divider packets
    // We add 1 to convert from 0-based to 1-based indexing
    let index1 = packets.iter().position(|p| p == &divider1).unwrap() + 1;
    let index2 = packets.iter().position(|p| p == &divider2).unwrap() + 1;

    // Calculate decoder key
    let decoder_key = index1 * index2;

    println!("Day 13 - Part 2: {}", decoder_key);
}
