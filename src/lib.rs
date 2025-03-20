use sctype::scint::ScInt;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

mod sctype;

pub fn fibonacci(n: ScInt) -> ScInt {
    let dp: Arc<Mutex<HashMap<ScInt, ScInt>>> = Arc::new(Mutex::new(HashMap::new()));

    {
        let mut dp_lock = dp.lock().unwrap();
        dp_lock.insert(ScInt::from_i128(0), ScInt::from_i128(0));
        dp_lock.insert(ScInt::from_i128(1), ScInt::from_i128(1));
        dp_lock.insert(ScInt::from_i128(2), ScInt::from_i128(1));
    }

    fib(&dp, &n)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = fibonacci(ScInt::from_i128(1000));
        assert_eq!(a, sctype::scint::ScInt::from("43466557686937456435688527675040625802564660517371780402481729089536555417949051890403879840079255169295922593080322634775209689623239873322471161642996440906533187938298969649928516003704476137795166849228875").unwrap());
    }
}
