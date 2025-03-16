use sctype::scint::ScInt;

mod sctype;

pub fn add(left: &str, right: &str) -> ScInt {
    sctype::scint::ScInt::from(left).unwrap() + sctype::scint::ScInt::from(right).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = sctype::scint::ScInt::from("19929234843272348973248974328978342").unwrap();
        let b = sctype::scint::ScInt::from("274535830479054983728427509345302789").unwrap();
        let result = a + b;
        assert_eq!(
            result,
            sctype::scint::ScInt::from("294465065322327332701676483674281131").unwrap()
        );
    }
}
