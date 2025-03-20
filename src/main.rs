use sctype::scint::ScInt;
use std::collections::HashMap;
use std::io;
use std::sync::{Arc, Mutex};

mod sctype;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n = ScInt::from(buffer.trim()).unwrap();

    let dp: Arc<Mutex<HashMap<ScInt, ScInt>>> = Arc::new(Mutex::new(HashMap::new()));

    {
        let mut dp_lock = dp.lock().unwrap();
        dp_lock.insert(ScInt::from_i128(0), ScInt::from_i128(0));
        dp_lock.insert(ScInt::from_i128(1), ScInt::from_i128(1));
        dp_lock.insert(ScInt::from_i128(2), ScInt::from_i128(1));
    }

    println!("{}", fib(&dp, &n));
}

fn fib(dp: &Arc<Mutex<HashMap<ScInt, ScInt>>>, n: &ScInt) -> ScInt {
    {
        let dp_lock = dp.lock().unwrap();
        if let Some(x) = dp_lock.get(n) {
            return x.clone();
        }
    }

    let x = fib(dp, &(n - ScInt::from_i128(1))) + fib(dp, &(n - ScInt::from_i128(2)));
    let mut dp_lock = dp.lock().unwrap();
    dp_lock.insert(n.clone(), x.clone());

    x
}
