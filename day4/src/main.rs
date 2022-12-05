use itertools::Itertools;

use crate::aoc_file::{get_file_param, read_lines};

mod aoc_file;

fn main() {
    println!("Overlaps: {}", count_containments(&get_file_param()));
    println!("Intersections: {}", count_intersections(&get_file_param()));
}

#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
struct CleaningRange {
    start: i32,
    end: i32,
}

impl CleaningRange {
    fn new(start: i32, end: i32) -> Self {
        assert!(start <= end);
        Self { start, end }
    }

    fn contains_range(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn in_range(&self, value: i32) -> bool {
        self.start <= value && value <= self.end
    }

    fn intersects_range(&self, other: &Self) -> bool {
        self.in_range(other.start)
            || self.in_range(other.end)
            || other.in_range(self.start)
            || other.in_range(self.end)
    }
}

impl TryFrom<&str> for CleaningRange {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut values = value.split("-").map(|x| x.parse::<i32>());
        match (values.next(), values.next()) {
            (Some(Ok(start)), Some(Ok(end))) => Ok(CleaningRange::new(start, end)),
            (_, _) => Err(format!("Unable to parse {}", value)),
        }
    }
}

fn parse_line<T: AsRef<str>>(line: T) -> (CleaningRange, CleaningRange) {
    // Assumes the input is well behaved.
    line.as_ref()
        .split(",")
        .map(|x| CleaningRange::try_from(x).unwrap())
        .collect_tuple()
        .unwrap()
}

fn count_containments(filename: &str) -> i32 {
    read_lines(filename)
        .unwrap()
        .iter()
        .filter(|line| {
            let (first, second) = parse_line(line);
            first.contains_range(&second) || second.contains_range(&first)
        })
        .count() as i32
}

fn count_intersections(filename: &str) -> i32 {
    read_lines(filename)
        .unwrap()
        .iter()
        .filter(|line| {
            let (first, second) = parse_line(line);
            first.intersects_range(&second)
        })
        .count() as i32
}

#[test]
fn count_containments_test() {
    assert_eq!(count_containments("./test1.txt"), 2);
}

#[test]
fn contains_range_other_test() {
    // Fully contained
    assert!(CleaningRange::new(2, 8).contains_range(&CleaningRange::new(3, 7)));
    assert!(CleaningRange::new(4, 6).contains_range(&CleaningRange::new(6, 6)));

    assert!(!CleaningRange::new(2, 8).contains_range(&CleaningRange::new(3, 9)));
    assert!(!CleaningRange::new(4, 6).contains_range(&CleaningRange::new(7, 7)));
}

#[test]
fn try_from_test() {
    assert_eq!(CleaningRange::try_from("2-4"), Ok(CleaningRange::new(2, 4)));
    assert_eq!(CleaningRange::try_from("2-3"), Ok(CleaningRange::new(2, 3)));
    assert_eq!(CleaningRange::try_from("5-7"), Ok(CleaningRange::new(5, 7)));
}

#[test]
fn parse_line_test() {
    assert_eq!(
        parse_line("2-4,6-8"),
        (CleaningRange::new(2, 4), CleaningRange::new(6, 8))
    );
}

#[test]
fn intersects_range_test() {
    assert!(
        CleaningRange::new(10, 20).intersects_range(&CleaningRange::new(5, 10)),
        "range touching at start"
    );
    assert!(
        !CleaningRange::new(10, 20).intersects_range(&CleaningRange::new(5, 9)),
        "second range before"
    );

    assert!(
        CleaningRange::new(10, 20).intersects_range(&CleaningRange::new(20, 25)),
        "range touching at end"
    );
    assert!(
        !CleaningRange::new(10, 20).intersects_range(&CleaningRange::new(21, 25)),
        "second range after"
    );

    assert!(
        CleaningRange::new(10, 20).intersects_range(&CleaningRange::new(5, 25)),
        "first range totally contained in second"
    );

    assert!(
        CleaningRange::new(10, 20).intersects_range(&CleaningRange::new(15, 19)),
        "second range totally contained in first"
    );
}

#[test]
fn count_intersections_test() {
    assert_eq!(count_intersections("./test1.txt"), 4);
}
/*
For example, consider the following list of section assignment pairs:

2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
For the first few pairs, this list means:

Within the first pair of Elves, the first Elf was assigned sections 2-4 (sections 2, 3, and 4), while the second Elf was assigned sections 6-8 (sections 6, 7, 8).
The Elves in the second pair were each assigned two sections.
The Elves in the third pair were each assigned three sections: one got sections 5, 6, and 7, while the other also got 7, plus 8 and 9.
This example list uses single-digit section IDs to make it easier to draw; your actual list might contain larger numbers. Visually, these pairs of section assignments look like this:

.234.....  2-4
.....678.  6-8

.23......  2-3
...45....  4-5

....567..  5-7
......789  7-9

.2345678.  2-8
..34567..  3-7

.....6...  6-6
...456...  4-6

.23456...  2-6
...45678.  4-8

Some of the pairs have noticed that one of their assignments fully contains the other. For example, 2-8 fully contains 3-7, and 6-6 is fully contained by 4-6. In pairs where one assignment fully contains the other, one Elf in the pair would be exclusively cleaning sections their partner will already be cleaning, so these seem like the most in need of reconsideration. In this example, there are 2 such pairs. */
