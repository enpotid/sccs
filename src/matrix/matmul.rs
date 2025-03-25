use std::ops::{AddAssign, Mul};

use crate::sctype::scmatrix::ScMatrix2D;

pub fn mul2d<T: Default + Clone + Mul<Output = T> + AddAssign>(
    a: ScMatrix2D<T>,
    b: ScMatrix2D<T>,
) -> Option<ScMatrix2D<T>> {
    let a_rows = a.rows;
    let a_cols = a.cols;
    let b_rows = b.rows;
    let b_cols = b.cols;

    if a_cols != b_rows {
        return None;
    }

    let mut result = vec![vec![T::default().clone(); b_cols]; a_rows];

    for i in 0..a_rows {
        for j in 0..b_cols {
            for k in 0..a_cols {
                result[i][j] += a.value[i][k].clone() * b.value[k][j].clone();
            }
        }
    }

    Some(ScMatrix2D {
        rows: a_rows,
        cols: b_cols,
        value: result,
    })
}
