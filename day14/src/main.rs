mod aoc_file;
mod matrix;

use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use aoc_file::read_lines;
use itertools::Itertools;
use matrix::Matrix;
use std::ops::{Range, Sub};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Default)]
struct Point(usize, usize);
impl Point {
    fn line_expand(range: Range<&'_ Point>) -> impl Iterator<Item = Point> + '_ {
        let first = range.end.min(range.start);
        let second = range.end.max(range.start);

        (first.0.clone()..=second.0)
            .flat_map(move |row| (first.1..=second.1).map(move |col| Point(row, col)))
    }
}

impl Display for Matrix<MapCell> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for row in 0..self.height {
            for i in self.row(row) {
                s.push(match i {
                    MapCell::Air => '.',
                    MapCell::Rock => '#',
                    MapCell::Sand => 'o',
                });
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}
struct RockFormation(Vec<Point>);

struct MapSpec {
    upper_left: Point,
    width: usize,
    height: usize,
    rock_formations: Vec<RockFormation>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MapCell {
    Air,
    Rock,
    Sand,
}

impl Default for MapCell {
    fn default() -> Self {
        MapCell::Air
    }
}

struct Map {
    map_spec: MapSpec,
    data: Matrix<MapCell>,
}

impl Map {
    fn compute_offset(datum: &Point, point: &Point) -> [usize; 2] {
        let row = point.0 - datum.0;
        let col = point.1 - datum.1;

        [row, col]
    }
}

impl Index<[usize; 2]> for Map {
    type Output = MapCell;

    fn index(&self, [row, col]: [usize; 2]) -> &Self::Output {
        let row = row - self.map_spec.upper_left.0;
        let col = col - self.map_spec.upper_left.1;

        &self.data[[row, col]]
    }
}

impl Index<Point> for Map {
    type Output = MapCell;

    fn index(&self, index: Point) -> &Self::Output {
        &self.data[[index.0, index.1]]
    }
}

impl IndexMut<[usize; 2]> for Map {
    fn index_mut(&mut self, [row, col]: [usize; 2]) -> &mut Self::Output {
        let row = row - self.map_spec.upper_left.0;
        let col = col - self.map_spec.upper_left.1;

        &mut self.data[[row, col]]
    }
}

impl IndexMut<Point> for Map {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self[[index.0, index.1]]
    }
}

impl From<MapSpec> for Map {
    fn from(map_spec: MapSpec) -> Self {
        let mut m = Map {
            data: Matrix::new(map_spec.width, map_spec.height),
            map_spec,
        };

        for RockFormation(formation) in m.map_spec.rock_formations.iter() {
            println!("{}", &m.data);
            for window in formation.windows(2) {
                for p in Point::line_expand(&window[0]..&window[1]) {
                    m.data[dbg!(Map::compute_offset(&m.map_spec.upper_left, &dbg!(p)))] =
                        MapCell::Rock;
                }
            }
        }

        m
    }
}

fn main() {
    let _formations = parse_file_to_structure_definitions(&aoc_file::get_file_param());
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

    let width = &lower_right.0 - &upper_left.0;
    let height = &lower_right.1 - &upper_left.1;

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
    assert_eq!(width, 9);
    assert_eq!(height, 5);
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
