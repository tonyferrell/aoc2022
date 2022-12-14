use std::ops::{Index, IndexMut};

pub struct MatrixIndex {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Matrix<T> {
    // interesting_window: Option<Range<usize>>,
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
        let end_idx = &start_idx + self.width - 1;

        &self.data[start_idx..end_idx]
    }
    pub fn new(height: usize, width: usize) -> Self {
        Matrix {
            // interesting_window: None,
            height,
            width,
            data: vec![Default::default(); width * height],
        }
    }
}
impl From<[usize; 2]> for MatrixIndex {
    fn from([row, col]: [usize; 2]) -> Self {
        MatrixIndex {
            row,
            col,
        }
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
            height: 3,
            width: 3,
            data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
        }
    )
}
