use crate::matrix::matmul;
use sctype::scmatrix::ScMatrix2D;

mod matrix;
mod sctype;

fn main() {
    let a = ScMatrix2D::from(vec![vec![1, 2], vec![4, 5]]);
    let b = ScMatrix2D::from(vec![vec![1], vec![2]]);

    println!("{:?}", matmul::mul2d(a, b).unwrap());
}
