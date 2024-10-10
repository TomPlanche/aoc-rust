///
/// # day_07.rs
/// Code for the day 01 of the Advent of Code challenge year 2022
///
// Imports  ==============================================================================  Imports
use std::collections::HashMap;

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../../data/year_2022/inputs/day_07.txt");

///
/// # FileSystem
///
/// Simulate a file system.
struct FileSystem {
    directories: HashMap<String, Directory>,
}

///
/// # Directory
///
/// Represent a directory in the file system.
struct Directory {
    size: usize,
    parent: Option<String>,
}

impl FileSystem {
    fn new() -> Self {
        let mut fs = FileSystem {
            directories: HashMap::new(),
        };
        fs.directories.insert(
            "/".to_string(),
            Directory {
                size: 0,
                parent: None,
            },
        );
        fs
    }

    ///
    /// # add_file
    ///
    /// Add a file to the file system.
    ///
    /// ## Arguments
    ///
    /// * `path` - The path of the file.
    /// * `size` - The size of the file.
    ///
    fn add_file(&mut self, path: &str, size: usize) {
        let mut current = path.to_string();
        while let Some(dir) = self.directories.get_mut(&current) {
            dir.size += size;
            if let Some(parent) = &dir.parent {
                current = parent.clone();
            } else {
                break;
            }
        }
    }

    ///
    /// # add_dir
    ///
    /// Add a directory to the file system.
    ///
    /// ## Arguments
    ///
    /// * `path` - The path of the directory.
    fn add_dir(&mut self, path: &str, name: &str) {
        let full_path = if path == "/" {
            format!("/{}", name)
        } else {
            format!("{}/{}", path, name)
        };
        self.directories.insert(
            full_path.clone(),
            Directory {
                size: 0,
                parent: Some(path.to_string()),
            },
        );
    }
}

// Functions  =========================================================================== Functions
pub fn response_part_1() {
    println!("Day 07 - Part 1");

    let mut fs = FileSystem::new();
    let mut current_path = "/".to_string();

    for line in INPUT.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[0] {
            "$" => {
                if parts[1] == "cd" {
                    match parts[2] {
                        "/" => current_path = "/".to_string(),
                        ".." => {
                            if let Some(parent) = &fs.directories[&current_path].parent {
                                current_path = parent.clone();
                            }
                        }
                        dir => {
                            current_path = if current_path == "/" {
                                format!("/{}", dir)
                            } else {
                                format!("{}/{}", current_path, dir)
                            };
                        }
                    }
                }
            }
            "dir" => fs.add_dir(&current_path, parts[1]),
            size => {
                if let Ok(file_size) = size.parse::<usize>() {
                    fs.add_file(&current_path, file_size);
                }
            }
        }
    }

    let sum: usize = fs
        .directories
        .values()
        .filter(|dir| dir.size <= 100000)
        .map(|dir| dir.size)
        .sum();

    println!(
        "Sum of total sizes of directories with size at most 100000: {}",
        sum
    );
}

pub fn response_part_2() {
    println!("Day 07 - Part 2");

    let mut fs = FileSystem::new();
    let mut current_path = "/".to_string();

    // (same as in part 1)
    for line in INPUT.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[0] {
            "$" => {
                if parts[1] == "cd" {
                    match parts[2] {
                        "/" => current_path = "/".to_string(),
                        ".." => {
                            if let Some(parent) = &fs.directories[&current_path].parent {
                                current_path = parent.clone();
                            }
                        }
                        dir => {
                            current_path = if current_path == "/" {
                                format!("/{}", dir)
                            } else {
                                format!("{}/{}", current_path, dir)
                            };
                        }
                    }
                }
            }
            "dir" => fs.add_dir(&current_path, parts[1]),
            size => {
                if let Ok(file_size) = size.parse::<usize>() {
                    fs.add_file(&current_path, file_size);
                }
            }
        }
    }

    const TOTAL_DISK_SPACE: usize = 70000000;
    const REQUIRED_UNUSED_SPACE: usize = 30000000;

    let used_space = fs.directories["/"].size;
    let unused_space = TOTAL_DISK_SPACE - used_space;
    let space_to_free = REQUIRED_UNUSED_SPACE - unused_space;

    let smallest_sufficient_dir = fs
        .directories
        .values()
        .filter(|dir| dir.size >= space_to_free)
        .min_by_key(|dir| dir.size)
        .unwrap();

    println!(
        "Size of the smallest directory that, if deleted, would free up enough space: {}",
        smallest_sufficient_dir.size
    );
}
