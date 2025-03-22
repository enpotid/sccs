use sctype::scint::ScInt;
use std::io;

mod sctype;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n = ScInt::from(buffer.trim()).unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let m: ScInt = ScInt::from(buffer.trim()).unwrap();

    println!("{}", n * m);
}
