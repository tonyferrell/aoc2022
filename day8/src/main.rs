mod aoc_file;

use std::collections::HashMap;

fn main() {
    let visible = count_trees_1(load_file(&aoc_file::get_file_param()));
    println!("There are {} trees visible", visible);
}

#[derive(Debug, PartialEq, Eq)]
enum Tree {
    Visible {
        height: usize,
        scenic_score: Option<usize>,
        visibility: HashMap<Direction, Option<usize>>,
    },
    Unvisited(usize),
}

use std::cmp;
use Tree::{Unvisited, Visible};

fn load_file(filename: &str) -> Vec<Vec<Tree>> {
    let lines = aoc_file::read_lines(filename).unwrap();

    lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| Unvisited(c.to_string().parse().unwrap()))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>()
}
fn print_forest(forest: &Vec<Vec<Tree>>) {
    for row in forest {
        let mut output = String::new();
        for tree in row {
            let next = match tree {
                Unvisited(_) => ".".to_owned(),
                Visible { height, .. } => format!("{}", height),
            };
            output += &next;
        }

        println!("{}", output);
    }
}
fn count_trees_1(mut forest: Vec<Vec<Tree>>) -> usize {
    // Like in physics, we will start by assuming a perfectly rectangular forrest.
    let row_count = forest.len();
    let col_count = forest[0].len();

    print_forest(&forest);

    let top_down_range = 0..row_count;
    let bottom_up_range = top_down_range.clone().rev();
    let left_right_range = 0..col_count;
    let right_left_range = left_right_range.clone().rev();

    // Walk each tree
    // Check the visibility on each of the compass directions.
    // If the visiting tree

    // From left
    for r in top_down_range.clone() {
        let mut max: Option<usize> = None;
        for c in left_right_range.clone() {
            update_state(&mut forest, r, c, &mut max);
        }
    }

    // From right
    for r in top_down_range.clone() {
        let mut max: Option<usize> = None;
        for c in right_left_range.clone() {
            update_state(&mut forest, r, c, &mut max);
        }
    }

    // From bottom
    for c in left_right_range.clone() {
        let mut max: Option<usize> = None;
        for r in bottom_up_range.clone() {
            update_state(&mut forest, r, c, &mut max);
        }
    }

    // From top
    for c in left_right_range.clone() {
        let mut max: Option<usize> = None;
        for r in top_down_range.clone() {
            update_state(&mut forest, r, c, &mut max);
        }
    }

    println!("After: ");
    print_forest(&forest);
    forest
        .iter()
        .flat_map(|row| {
            row.iter().filter(|&x| match x {
                Visible { .. } => true,
                _ => false,
            })
        })
        .count()
}
fn update_state(forest: &mut Vec<Vec<Tree>>, row: usize, col: usize, max: &mut Option<usize>) {
    let t = match forest[row][col] {
        Visible { height, .. } => {
            // Tree is already visible. All we need to do is see if it's taller than the current max.
            *max = Some(cmp::max(max.unwrap_or(height), height));
            Visible {
                height,
                scenic_score: None,
                visibility: HashMap::new(),
            }
        }
        Unvisited(height) => {
            let curr_max = max.unwrap_or_default();
            let edge_tree = match max {
                None => true,
                _ => false,
            };
            if edge_tree || height > curr_max {
                *max = Some(height);
                Visible {
                    height,
                    scenic_score: None,
                    visibility: HashMap::new(),
                }
            } else {
                Unvisited(height)
            }
        }
    };

    forest[row][col] = t;
}

fn update_scenic(forest: &mut Vec<Vec<Tree>>) {
    // Like in physics, we will start by assuming a perfectly rectangular forrest.
    let row_count = forest.len();
    let col_count = forest[0].len();

    for r in 1..row_count {
        for c in 1..col_count {
            // Let start looking for some visibilies!
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn view(
    d: Direction,
    source_height: &usize,
    row: usize,
    col: usize,
    forest: &mut Vec<Vec<Tree>>,
) -> usize {
    let irow = row as i32;
    let icol = col as i32;
    let (new_row, new_col) = match d {
        Direction::Up => (irow - 1, icol),
        Direction::Down => (irow + 1, icol),
        Direction::Left => (irow, icol - 1),
        Direction::Right => (irow, icol + 1),
    };

    if new_row < 0
        || new_col < 0
        || new_row >= forest.len() as i32
        || new_col >= forest[0].len() as i32
    {
        // Edge of forest.
        return 0;
    } else {
        if let Unvisited(height) = &forest[new_row as usize][new_col as usize] {
            let continued_view = view(d, height, new_row as usize, new_col as usize, forest);
            let new_tree = Visible { height, scenic_score: None, visibility: HashMap::new() };

&mut forest[new_row][new_col] = new_tree;

        }
        let this_tree = &forest[row][col];

        match next_tree {
            // The next tree has been visited
            Visible {
                height, visibility, ..
            } => {
                if height < source_height {
                    return visibility
                        .entry(d)
                        .or_insert_with(|| {
                            Some(view(d, &height, new_row as usize, new_col as usize, forest))
                        })
                        .unwrap()
                        + 1;
                } else {
                    return 1;
                }
            }
            Unvisited(height) => {
                return view(d, &height, new_row as usize, new_col as usize, forest) + 1
            }
        }
    }
}
#[test]
fn update_scenic_test() {}

#[test]
fn load_file_test() {
    let file = load_file("./test.txt");
    let expected: Vec<Vec<usize>> = vec![
        vec![3, 0, 3, 7, 3],
        vec![2, 5, 5, 1, 2],
        vec![6, 5, 3, 3, 2],
        vec![3, 3, 5, 4, 9],
        vec![3, 5, 3, 9, 0],
    ];
    assert_eq!(
        file,
        expected
            .into_iter()
            .map(|row| row
                .into_iter()
                .map(|val| Unvisited(val))
                .collect::<Vec<_>>())
            .collect::<Vec<_>>()
    );
}

#[test]
fn input_file_test() {
    assert_eq!(count_trees_1(load_file("./test.txt")), 21);
}
