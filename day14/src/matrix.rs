use std::ops::{Index, IndexMut};

#[derive(Debug, PartialEq, Eq)]
pub struct MatrixIndex {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rectangle {
    upper_left: MatrixIndex,
    lower_right: MatrixIndex,
}

impl Rectangle {
    pub fn include(&self, new: MatrixIndex) -> Self {
        Rectangle {
            upper_left: MatrixIndex {
                row: self.upper_left.row.min(new.row),
                col: self.upper_left.col.min(new.col),
            },
            lower_right: MatrixIndex {
                row: self.lower_right.row.max(new.row),
                col: self.lower_right.col.max(new.col),
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Matrix<T> {
    pub interesting_window: Option<Rectangle>,
    pub height: usize,
    pub width: usize,
    data: Vec<T>,
}

impl<T> Matrix<T>
where
    T: Clone + Default,
{
    pub fn row(&self, row_number: usize) -> &[T] {
        let start_idx = self.width * row_number;
        let end_idx = &start_idx + self.width;

        &self.data[start_idx..end_idx]
    }
    pub fn new(height: usize, width: usize) -> Self {
        Matrix {
            interesting_window: None,
            height,
            width,
            data: vec![Default::default(); width * height],
        }
    }
}
impl From<[usize; 2]> for MatrixIndex {
    fn from([row, col]: [usize; 2]) -> Self {
        MatrixIndex { row, col }
    }
}

impl<T> Index<MatrixIndex> for Matrix<T>
where
    T: Clone + Default,
{
    type Output = T;

    fn index(&self, MatrixIndex { row, col }: MatrixIndex) -> &Self::Output {
        &self.data[row * self.width + col]
    }
}

impl<T> IndexMut<MatrixIndex> for Matrix<T>
where
    T: Clone + Default,
{
    fn index_mut(&mut self, MatrixIndex { row, col }: MatrixIndex) -> &mut Self::Output {
        self.interesting_window = Some(match &self.interesting_window {
            Some(current_window) => current_window.include(MatrixIndex { row, col }),
            None => Rectangle {
                upper_left: MatrixIndex { row, col },
                lower_right: MatrixIndex { row, col },
            },
        });

        &mut self.data[row * self.width + col]
    }
}

#[test]
fn mut_access_test() {
    let mut m = Matrix::new(3, 3);
    let mut i = 1;
    for r in 0..3 {
        for c in 0..3 {
            m[[r, c].into()] = i;
            i += 1;
        }
    }

    assert_eq!(
        m,
        Matrix {
            interesting_window: Some(Rectangle {
                upper_left: MatrixIndex { row: 0, col: 0 },
                lower_right: MatrixIndex { row: 2, col: 2 }
            }),
            height: 3,
            width: 3,
            data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
        }
    )
}
