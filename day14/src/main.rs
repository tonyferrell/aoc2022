mod aoc_file;
mod map;
mod matrix;
mod point;

use aoc_file::read_lines;
use itertools::Itertools;
use map::*;
use point::*;

use crate::matrix::MatrixIndex;

fn main() {
    let map: Map = parse_file_to_structure_definitions(&aoc_file::get_file_param())
        .unwrap()
        .into();
    println!("{}", map.data);
}

fn parse_file_to_structure_definitions(filename: &str) -> Result<MapSpec, ()> {
    let lines = read_lines(filename).ok_or(())?;
    let mut upper_left: Option<Point> = None;
    let mut lower_right: Option<Point> = None;

    let rock_formations = lines
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
                        upper_left = match &upper_left {
                            None => Some(Point(x, y)),
                            Some(ul) => Some(Point(ul.0.min(x), ul.1.min(y))),
                        };
                        lower_right = match &lower_right {
                            None => Some(Point(x, y)),
                            Some(ul) => Some(Point(ul.0.max(x), ul.1.max(y))),
                        };
                        Point(x, y)
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    let upper_left = upper_left.unwrap_or_default();
    let lower_right = lower_right.unwrap_or_default();

    let width = &lower_right.0 - &upper_left.0 + 1;
    let height = &lower_right.1 - &upper_left.1 + 1;

    Ok(MapSpec {
        upper_left,
        width,
        height,
        rock_formations,
    })
}

#[test]
fn test_map_parse_from_spec() {
    let map: Map = parse_file_to_structure_definitions("./test.txt")
        .expect("file should be readable")
        .into();
}

#[test]
fn parse_file_structures_test() {
    let MapSpec {
        upper_left,
        width,
        height,
        rock_formations,
    } = parse_file_to_structure_definitions("./test.txt").expect("file should be readable");

    assert_eq!(upper_left, Point(494, 4));
    assert_eq!(width, 10);
    assert_eq!(height, 6);
    assert_eq!(rock_formations.len(), 2);
    assert_eq!(
        rock_formations[0].0,
        vec![Point(498, 4), Point(498, 6), Point(496, 6)]
    );

    assert_eq!(
        rock_formations[1].0,
        vec![Point(503, 4), Point(502, 4), Point(502, 9), Point(494, 9)]
    );
}

#[test]
fn point_ordering_test() {
    let one = Point(100, 1);
    let two = Point(100, 2);

    assert!(one < two);
    assert!(two > one);

    let one = Point(100, 0);
    let two = Point(101, 0);

    assert!(one < two);
    assert!(two > one);

    assert_eq!(one.max(two.clone()), two);
}

#[test]
fn line_generation_test() {
    /* 498,4 -> 498,6 -> 496,6 */
    assert_eq!(
        Point::line_expand(&Point(498, 4)..&Point(498, 6)).collect::<Vec<_>>(),
        vec![Point(498, 4), Point(498, 5), Point(498, 6)]
    );

    assert_eq!(
        Point::line_expand(&Point(498, 6)..&Point(496, 6)).collect::<Vec<_>>(),
        vec![Point(496, 6), Point(497, 6), Point(498, 6)]
    );
    // 503,4 -> 502,4 -> 502,9 -> 494,9
    assert_eq!(
        Point::line_expand(&Point(503, 4)..&Point(502, 4)).collect::<Vec<_>>(),
        vec![Point(502, 4), Point(503, 4),]
    );
    assert_eq!(
        Point::line_expand(&Point(502, 4)..&Point(502, 9)).collect::<Vec<_>>(),
        vec![
            Point(502, 4),
            Point(502, 5),
            Point(502, 6),
            Point(502, 7),
            Point(502, 8),
            Point(502, 9),
        ]
    );
    assert_eq!(
        Point::line_expand(&Point(502, 9)..&Point(494, 9)).collect::<Vec<_>>(),
        vec![
            Point(494, 9),
            Point(495, 9),
            Point(496, 9),
            Point(497, 9),
            Point(498, 9),
            Point(499, 9),
            Point(500, 9),
            Point(501, 9),
            Point(502, 9),
        ]
    );
}

#[test]
fn compute_offsets_test() {
    let datum = Point(494, 4);
    assert_eq!(Map::compute_offset(&datum, &Point(494, 4)), Point(0, 0));
    assert_eq!(Map::compute_offset(&datum, &Point(503, 9)), Point(9, 5));
    assert_eq!(Map::compute_offset(&datum, &Point(495, 6)), Point(1, 2));
}
