///
/// # day_08.rs
/// Code for the day 01 of the Advent of Code challenge year 2022
///

// Imports  ==============================================================================  Imports

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../../data/year_2022/inputs/day_08.txt");

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Tree {
    position: (usize, usize),
    height: usize,
}

struct Grid {
    trees: Vec<Vec<Tree>>,
}

impl Tree {
    fn new(position: (usize, usize), height: usize) -> Self {
        Tree { position, height }
    }

    ///
    /// # is_visible
    ///
    /// A tree is visible if all of the other trees between it and an edge of the grid are shorter than it.
    ///
    /// ## Arguments
    ///
    /// * `trees` - A vector of trees.
    /// * `from` - The tree to check visibility from.
    ///
    /// ## Returns
    ///
    /// * True if the tree is visible, false otherwise.
    fn is_visible(&self, trees: &Vec<Vec<Tree>>, from: Direction) -> bool {
        let (row, col) = self.position;
        let height = self.height;

        match from {
            // Check if all the trees in the direction are shorter than the current tree.
            Direction::Up => (0..row).all(|r| trees[r][col].height < height),
            Direction::Down => (row + 1..trees.len()).all(|r| trees[r][col].height < height),
            Direction::Left => (0..col).all(|c| trees[row][c].height < height),
            Direction::Right => (col + 1..trees[row].len()).all(|c| trees[row][c].height < height),
        }
    }

    ///
    /// # viewing_distance
    ///
    /// Calculate the viewing distance of a tree in a given direction.
    ///
    /// ## Arguments
    ///
    /// * `trees` - A vector of trees.
    /// * `direction` - The direction to calculate the viewing distance.
    ///
    /// ## Returns
    ///
    /// * The viewing distance.
    fn viewing_distance(&self, trees: &Vec<Vec<Tree>>, direction: Direction) -> usize {
        let (row, col) = self.position;
        let height = self.height;
        let mut distance = 0;

        match direction {
            Direction::Up => {
                for r in (0..row).rev() {
                    distance += 1;
                    if trees[r][col].height >= height {
                        break;
                    }
                }
            }
            Direction::Down => {
                for r in row + 1..trees.len() {
                    distance += 1;
                    if trees[r][col].height >= height {
                        break;
                    }
                }
            }
            Direction::Left => {
                for c in (0..col).rev() {
                    distance += 1;
                    if trees[row][c].height >= height {
                        break;
                    }
                }
            }
            Direction::Right => {
                for c in col + 1..trees[row].len() {
                    distance += 1;
                    if trees[row][c].height >= height {
                        break;
                    }
                }
            }
        }

        distance
    }

    fn scenic_score(&self, trees: &Vec<Vec<Tree>>) -> usize {
        self.viewing_distance(trees, Direction::Up)
            * self.viewing_distance(trees, Direction::Down)
            * self.viewing_distance(trees, Direction::Left)
            * self.viewing_distance(trees, Direction::Right)
    }
}

impl Grid {
    ///
    /// # new
    ///
    /// Create a new grid from a &str.
    ///
    /// ## Arguments
    ///
    /// * `input` - The input string.
    ///
    /// ## Returns
    ///
    /// * A new Grid.
    fn new(input: &str) -> Self {
        let mut trees = Vec::new();

        for (i, line) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (j, height) in line.chars().enumerate() {
                let height = height.to_digit(10).unwrap() as usize;
                row.push(Tree::new((i, j), height));
            }
            trees.push(row);
        }

        Grid { trees }
    }

    ///
    /// # get_all_visible_trees
    ///
    /// Get all the visible trees from a given tree.
    ///
    /// ## Returns
    ///
    /// * A vector of visible trees.
    fn get_all_visible_trees(&self) -> Vec<&Tree> {
        let mut visible_trees = Vec::new();

        for row in self.trees.iter() {
            for tree in row.iter() {
                if tree.is_visible(&self.trees, Direction::Up)
                    || tree.is_visible(&self.trees, Direction::Down)
                    || tree.is_visible(&self.trees, Direction::Left)
                    || tree.is_visible(&self.trees, Direction::Right)
                {
                    visible_trees.push(tree);
                }
            }
        }

        visible_trees
    }

    fn highest_scenic_score(&self) -> usize {
        let mut max_score = 0;

        for row in &self.trees {
            for tree in row {
                let score = tree.scenic_score(&self.trees);
                if score > max_score {
                    max_score = score;
                }
            }
        }

        max_score
    }
}

// Functions  =========================================================================== Functions
pub fn response_part_1() {
    println!("Day 08 - Part 1");

    let trees = Grid::new(INPUT);

    println!(
        "Number of visible trees: {}",
        trees.get_all_visible_trees().len()
    );
}

pub fn response_part_2() {
    println!("Day 08 - Part 2");

    let grid = Grid::new(INPUT);
    let highest_score = grid.highest_scenic_score();

    println!("The highest scenic score possible is: {}", highest_score);
}
