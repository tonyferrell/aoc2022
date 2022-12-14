use std::ops::{Index, IndexMut};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Matrix<T> {
    height: usize,
    width: usize,
    data: Vec<T>,
}

impl<T> Matrix<T>
where
    T: Clone + Default,
{
    fn new(height: usize, width: usize) -> Self {
        Matrix {
            height,
            width,
            data: vec![Default::default(); width * height],
        }
    }
}

impl<T> Index<[usize; 2]> for Matrix<T>
where
    T: Clone + Default,
{
    type Output = T;

    fn index(&self, [x, y]: [usize; 2]) -> &Self::Output {
        &self.data[x * self.width + y]
    }
}

impl<T> IndexMut<[usize; 2]> for Matrix<T>
where
    T: Clone + Default,
{
    fn index_mut(&mut self, [x, y]: [usize; 2]) -> &mut Self::Output {
        &mut self.data[x * self.width + y]
    }
}

#[test]
fn mut_access_test() {
    let mut m = Matrix::new(3, 3);
    let mut i = 1;
    for r in 0..3 {
        for c in 0..3 {
            m[[r, c]] = i;
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
