//! Instructions are there: https://adventofcode.com/2022/day/8

use std::error::Error;
use std::io::{BufRead, BufReader, Read, Seek};

use utils::{CodeSolution, SantaError};

type TreeHeight = u8;
type ViewScore = u32;

pub struct DailySolution;

impl CodeSolution for DailySolution {
    fn run<I>(input: I) -> Result<(), Box<dyn Error>>
    where
        I: Read + Seek,
    {
        let buf_input = BufReader::new(input);
        let forest = parse_forest(buf_input)?;

        let total_visible_trees = part_1(&forest);
        println!("Total visible trees: {}", total_visible_trees);

        let ((x, y), best_view_score) = part_2(&forest);
        println!("Best view score at [{};{}]: {}", x, y, best_view_score);

        Ok(())
    }
}

fn part_1(forest: &Forest) -> u16 {
    forest.get_visible_trees().len() as u16
}

fn part_2(forest: &Forest) -> ((usize, usize), ViewScore) {
    forest.get_best_scenic_score()
}

struct Forest {
    grid: Vec<Vec<TreeHeight>>,
}

impl Forest {
    fn new() -> Self {
        Self { grid: Vec::new() }
    }

    fn tree_is_on_edge(&self, tree_row: usize, tree_column: usize) -> bool {
        // Tree is on the edge
        tree_row == 0
            || tree_column == 0
            || tree_row == (self.grid.len() - 1)
            || tree_column == (self.grid[tree_row].len() - 1)
    }

    fn get_best_scenic_score(&self) -> ((usize, usize), ViewScore) {
        (1..self.grid.len() - 1)
            .flat_map(|row_num| {
                let row_len = self.grid[row_num].len();
                (1..row_len - 1).map(move |column_num| {
                    (
                        (row_num, column_num),
                        self.get_tree_scenic_score(row_num, column_num),
                    )
                })
            })
            .max_by_key(|(_, score)| *score)
            .unwrap()
    }

    fn get_tree_scenic_score(&self, tree_row: usize, tree_column: usize) -> ViewScore {
        if self.tree_is_on_edge(tree_row, tree_column) {
            return 0;
        }

        let mut total_score = 1;

        let tree_height = self.grid[tree_row][tree_column];

        total_score *= self.get_view_score(
            tree_height,
            self.grid[tree_row][..tree_column].iter().copied().rev(),
        );
        total_score *= self.get_view_score(
            tree_height,
            self.grid[tree_row][(tree_column + 1)..].iter().copied(),
        );
        total_score *= self.get_view_score(
            tree_height,
            self.grid[..tree_row]
                .iter()
                .map(|row| row[tree_column])
                .rev(),
        );
        total_score *= self.get_view_score(
            tree_height,
            self.grid[(tree_row + 1)..]
                .iter()
                .map(|row| row[tree_column]),
        );

        total_score
    }

    fn get_view_score<I>(&self, tree_height: TreeHeight, direction: I) -> ViewScore
    where
        I: Iterator<Item = TreeHeight>,
    {
        let mut current_view_score = 0;

        for other_tree in direction {
            current_view_score += 1;

            if other_tree >= tree_height {
                break;
            }
        }

        current_view_score
    }

    fn get_visible_trees(&self) -> Vec<TreeHeight> {
        let mut trees = Vec::new();

        for (row_num, row) in self.grid.iter().enumerate() {
            for (column_num, _) in row.iter().enumerate() {
                if self.tree_is_visible(row_num, column_num) {
                    trees.push(row[column_num]);
                }
            }
        }

        trees
    }

    fn tree_is_visible(&self, tree_row: usize, tree_column: usize) -> bool {
        // Tree is on the edge
        if self.tree_is_on_edge(tree_row, tree_column) {
            return true;
        }

        let tree_height = self.grid[tree_row][tree_column];

        // Check line
        let invisible_in_row_before = self.grid[tree_row][..tree_column]
            .iter()
            .any(|&other_tree| other_tree >= tree_height);
        let invisible_in_row_after = self.grid[tree_row][(tree_column + 1)..]
            .iter()
            .any(|&other_tree| other_tree >= tree_height);

        // Check column
        let invisible_in_column_before = (0..tree_row).into_iter().any(|row| {
            let other_tree = self.grid[row][tree_column];
            other_tree >= tree_height
        });
        let invisible_in_column_after = ((tree_row + 1)..self.grid.len()).into_iter().any(|row| {
            let other_tree = self.grid[row][tree_column];
            other_tree >= tree_height
        });

        let invisible_in_row = invisible_in_row_before && invisible_in_row_after;
        let invisible_in_column = invisible_in_column_before && invisible_in_column_after;

        !(invisible_in_row && invisible_in_column)
    }

    fn new_row(&mut self) {
        let row_size = match self.grid.last() {
            Some(row) => row.len(),
            None => 0,
        };
        self.grid.push(Vec::with_capacity(row_size));
    }

    fn push_tree(&mut self, new_tree: TreeHeight) {
        self.grid.last_mut().unwrap().push(new_tree);
    }
}

fn parse_forest<I>(input: I) -> Result<Forest, Box<dyn Error>>
where
    I: BufRead,
{
    let mut forest = Forest::new();

    for line in input.lines().filter_map(|x| x.ok()) {
        forest.new_row();

        for tree in line.chars() {
            let new_tree = tree.to_digit(10).ok_or_else(|| {
                SantaError::InvalidInput(format!("Invalid tree height '{}' in input.", tree))
            })? as TreeHeight;
            forest.push_tree(new_tree);
        }
    }

    Ok(forest)
}

#[cfg(test)]
mod test_day {
    use std::io::{BufReader, Cursor};

    use utils::setup_log;

    use super::*;

    const SAMPLE_INPUT: &str = r#"30373
25512
65332
33549
35390"#;

    #[test]
    fn test_visible_tree() {
        setup_log();

        let input_cursor = BufReader::new(Cursor::new(SAMPLE_INPUT));
        let forest = parse_forest(input_cursor).unwrap();

        assert!(forest.tree_is_visible(1, 1));
        assert!(forest.tree_is_visible(1, 2));
        assert!(!forest.tree_is_visible(1, 3));
    }

    #[test]
    fn test_view_score() {
        setup_log();

        let input_cursor = BufReader::new(Cursor::new(SAMPLE_INPUT));
        let forest = parse_forest(input_cursor).unwrap();

        assert_eq!(forest.get_tree_scenic_score(1, 2), 4);
        assert_eq!(forest.get_tree_scenic_score(3, 2), 8);
    }

    #[test]
    fn test_part_1() {
        setup_log();

        let input_cursor = BufReader::new(Cursor::new(SAMPLE_INPUT));
        let forest = parse_forest(input_cursor).unwrap();
        let visible_trees = forest.get_visible_trees().len();

        assert_eq!(visible_trees, 21);
    }

    #[test]
    fn test_part_2() {
        setup_log();

        let input_cursor = BufReader::new(Cursor::new(SAMPLE_INPUT));
        let forest = parse_forest(input_cursor).unwrap();
        let ((x, y), best_view_score) = part_2(&forest);

        assert_eq!(x, 3);
        assert_eq!(y, 2);
        assert_eq!(best_view_score, 8);
    }
}
