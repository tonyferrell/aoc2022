mod aoc_file;
mod matrix;

use aoc_file::read_lines;
use itertools::Itertools;
use matrix::Matrix;

#[derive(Debug, PartialEq, Eq)]
struct Point(usize, usize);

struct RockFormation(Vec<Point>);

fn main() {
    println!("Hello, world!");
}

fn parse_file_to_structure_definitions(filename: &str) -> Result<Vec<RockFormation>, ()> {
    let lines = read_lines(filename).ok_or(())?;
    Ok(lines
        .iter()
        .map(|line| {
            RockFormation(
                line.split(" -> ")
                    .map(|pairs| {
                        let (x, y) = pairs
                            .split(",")
                            .map(|x| x.parse::<usize>().unwrap())
                            .collect_tuple()
                            .unwrap();
                        Point(x, y)
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>())
}

#[test]
fn parse_file_structures_test() {
    let structures =
        parse_file_to_structure_definitions("./test.txt").expect("file should be readable");

    assert_eq!(structures.len(), 2);
    assert_eq!(
        structures[0].0,
        vec![Point(498, 4), Point(498, 6), Point(496, 6)]
    );

    assert_eq!(
        structures[1].0,
        vec![Point(503, 4), Point(502, 4), Point(502, 9), Point(494, 9)]
    );
}
