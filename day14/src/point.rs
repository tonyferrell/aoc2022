use std::ops::{Range, Sub};

use crate::matrix::MatrixIndex;


#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Default)]
pub struct Point(pub usize, pub usize);
impl Point {
    pub fn line_expand(range: Range<&'_ Point>) -> impl Iterator<Item = Point> + '_ {
        let first = range.end.min(range.start);
        let second = range.end.max(range.start);

        (first.0.clone()..=second.0)
            .flat_map(move |row| (first.1..=second.1).map(move |col| Point(row, col)))
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl From<Point> for MatrixIndex {
    fn from(Point(col, row): Point) -> Self {
        MatrixIndex { row, col }
    }
}