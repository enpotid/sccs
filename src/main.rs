use crate::matrix::matmul;
use sctype::scmatrix::ScMatrix2D;

mod matrix;
mod sctype;

fn main() {
    let a = ScMatrix2D::from(vec![vec![1, 2, 3], vec![4, 5, 6]]);
    let b = ScMatrix2D::from(vec![vec![7, 8], vec![9, 10], vec![11, 12]]);

    println!("{:?}", matmul::mul2d(a, b).unwrap());
}
