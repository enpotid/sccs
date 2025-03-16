use sctype::scint::ScInt;

mod sctype;

fn main() {
    let a = ScInt::from("543435435435555555555726892567980652498545705624798056247852678952678956489706524098754638970543092843876924537896453987564546657675465746574").unwrap();
    let b = ScInt::from("827809254087945089754897089076567565732346897234987231479843129873142986615680942530987246307982670892647089643780964708962478926487906240798").unwrap();
    println!("{}", a - b);
}
