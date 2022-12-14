use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use crate::{
    matrix::{Matrix, MatrixIndex, Rectangle},
    point::Point,
};

impl Display for MapCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MapCell::Air => '.',
                MapCell::Rock => '#',
                MapCell::Sand => 'o',
                MapCell::Entrance => '+',
            }
        )
    }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}-{:?}", self.upper_left, self.lower_right)
    }
}
impl Display for Matrix<MapCell> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(window) = &self.interesting_window {
            writeln!(
                f,
                "({}x{}) Matrix. Viewing ({})",
                &self.height, &self.width, &window
            )?;

            for row in window.upper_left.row..=window.lower_right.row {
                for col in window.upper_left.col..=window.lower_right.col {
                    write!(f, "{}", &self[MatrixIndex { row, col }])?;
                }
                write!(f, "\n")?;
            }
        } else {
            writeln!(
                f,
                "({}x{}) Matrix. Viewing (ALL)",
                &self.height, &self.width
            )?;

            for row in 0..self.height {
                for i in self.row(row) {
                    write!(f, "{}", i)?;
                }
                write!(f, "\n")?;
            }
        }

        Ok(())
    }
}
pub struct RockFormation(pub Vec<Point>);

pub struct MapSpec {
    pub width: usize,
    pub height: usize,
    pub rock_formations: Vec<RockFormation>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapCell {
    Air,
    Rock,
    Sand,
    Entrance,
}

impl Default for MapCell {
    fn default() -> Self {
        MapCell::Air
    }
}

pub struct Map {
    pub map_spec: MapSpec,
    pub data: Matrix<MapCell>,
}

impl Index<[usize; 2]> for Map {
    type Output = MapCell;

    fn index(&self, [row, col]: [usize; 2]) -> &Self::Output {
        &self.data[MatrixIndex { row, col }]
    }
}

impl Index<Point> for Map {
    type Output = MapCell;

    fn index(&self, index: Point) -> &Self::Output {
        let mi: MatrixIndex = index.into();
        &self.data[&mi]
    }
}

impl Index<&Point> for Map {
    type Output = MapCell;

    fn index(&self, index: &Point) -> &Self::Output {
        let mi: MatrixIndex = (*index).into();
        &self.data[&mi]
    }
}

impl IndexMut<Point> for Map {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        let mi: MatrixIndex = index.into();
        &mut self.data[mi]
    }
}

impl From<MapSpec> for Map {
    fn from(map_spec: MapSpec) -> Self {
        let mut m = Map {
            data: Matrix::new(dbg!(map_spec.height), dbg!(map_spec.width)),
            map_spec,
        };

        for RockFormation(formation) in m.map_spec.rock_formations.iter() {
            for window in formation.windows(2) {
                for p in Point::line_expand(&window[0]..&window[1]) {
                    m.data[p.into()] = MapCell::Rock;
                }
            }
        }

        m.data[Point(500, 0).into()] = MapCell::Entrance;

        m
    }
}
