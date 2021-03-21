use more_asserts::assert_lt;

pub struct Matrix<T> {
    data: Vec<T>,
    cols: usize,
    rows: usize,
}

impl<T> Matrix<T>
where
    T: Clone,
{
    pub fn new(cols: usize, rows: usize, default: T) -> Self {
        Matrix {
            data: vec![default; cols * rows],
            cols,
            rows,
        }
    }

    pub fn at(&self, col: usize, row: usize) -> &T {
        &self.data[self.index(col, row)]
    }

    pub fn mut_at(&mut self, col: usize, row: usize) -> &mut T {
        let index = self.index(col, row);
        &mut self.data[index]
    }

    fn index(&self, col: usize, row: usize) -> usize {
        assert_lt!(col, self.cols, "Column index out of range!");
        assert_lt!(row, self.rows, "Row index out of range!");
        col * self.rows + row
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Row index out of range!")]
    fn row_bounds_check_works() {
        Matrix::<i32>::new(5, 5, 0).at(0, 5);
    }

    #[test]
    #[should_panic(expected = "Column index out of range!")]
    fn column_bounds_check_works() {
        Matrix::<i32>::new(5, 5, 0).at(5, 0);
    }
}
