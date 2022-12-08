mod aoc_file;

fn main() {
    let visible = count_trees(load_file(&aoc_file::get_file_param()));
    println!("There are {} trees visible", visible);
}

#[derive(Debug, PartialEq, Eq)]
enum Tree {
    Visible(usize),
    Unknown(usize),
}

use std::cmp;
use Tree::{Unknown, Visible};

fn load_file(filename: &str) -> Vec<Vec<Tree>> {
    let lines = aoc_file::read_lines(filename).unwrap();

    lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| Unknown(c.to_string().parse().unwrap()))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>()
}
fn print_forest(forest: &Vec<Vec<Tree>>) {
    for row in forest {
        let mut output = String::new();
        for tree in row {
            let next = match tree {
                Unknown(_) => ".".to_owned(),
                Visible(height) => format!("{}", height),
            };
            output += &next;
        }

        println!("{}", output);
    }
}
fn count_trees(mut forest: Vec<Vec<Tree>>) -> usize {
    // Like in physics, we will start by assuming a perfectly rectangular forrest.
    let row_count = forest.len();
    let col_count = forest[0].len();

    print_forest(&forest);

    let top_down_range = 0..row_count;
    let bottom_up_range = top_down_range.clone().rev();
    let left_right_range = 0..col_count;
    let right_left_range = left_right_range.clone().rev();

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
                Visible(_) => true,
                _ => false,
            })
        })
        .count()
}

fn update_state(forest: &mut Vec<Vec<Tree>>, row: usize, col: usize, max: &mut Option<usize>) {
    let t = match forest[row][col] {
        Visible(height) => {
            // Tree is already visible. All we need to do is see if it's taller than the current max.
            *max = Some(cmp::max(max.unwrap_or(height), height));
            Visible(height)
        }
        Unknown(height) => {
            let curr_max = max.unwrap_or_default();
            let edge_tree = match max {
                None => true,
                _ => false,
            };
            if edge_tree || height > curr_max {
                *max = Some(height);
                Visible(height)
            } else {
                Unknown(height)
            }
        }
    };

    forest[row][col] = t;
}

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
            .map(|row| row.into_iter().map(|val| Unknown(val)).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    );
}

#[test]
fn count_trees_test() {}

#[test]
fn input_file_test() {
    assert_eq!(count_trees(load_file("./test.txt")), 21);
}
