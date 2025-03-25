use std::fmt::{self, Debug};

#[derive(PartialEq, Eq, Clone)]
pub struct ScMatrix2D<T> {
    pub rows: usize,
    pub cols: usize,
    pub value: Vec<Vec<T>>,
}

impl<T: Debug + Default + Clone> ScMatrix2D<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        let value = vec![vec![T::default().clone(); cols]; rows];
        ScMatrix2D { rows, cols, value }
    }

    pub fn from(value: Vec<Vec<T>>) -> Self {
        let rows = value.len();
        let cols = value.get(0).map_or(0, |row| row.len());
        ScMatrix2D { rows, cols, value }
    }
}

impl<T: Debug> fmt::Debug for ScMatrix2D<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "rows: {}, cols: {}, value: {:?}",
            self.rows, self.cols, self.value
        )
    }
}
