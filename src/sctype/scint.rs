use std::fmt;

#[derive(PartialEq, Eq)]
pub enum Sign {
    Plus,
    Minus,
}

pub struct ScInt {
    pub sign: Sign,
    pub value: Vec<u32>,
}

impl ScInt {
    pub fn new() -> Self {
        Self {
            sign: Sign::Plus,
            value: Vec::new(),
        }
    }

    pub fn from_u32(x: u32) -> Self {
        Self {
            sign: Sign::Plus,
            value: vec![x],
        }
    }

    pub fn from_u64(x: u64) -> Self {
        let n = 64;
        let mut s = format!("{:b}", x);
        let l = s.chars().collect::<Vec<char>>().len();
        s = format!("{}{}", "0".repeat(n - l), s);
        let mut list = Vec::new();
        let mut i = 0;
        for _ in 0..n / 32 {
            list.push(u32::from_str_radix(&s[i..i + 32].to_string(), 2).unwrap());
            i += 32
        }

        Self {
            sign: Sign::Plus,
            value: list,
        }
    }

    pub fn from_u128(x: u128) -> Self {
        let n = 128;
        let mut s = format!("{:b}", x);
        let l = s.chars().collect::<Vec<char>>().len();
        s = format!("{}{}", "0".repeat(n - l), s);
        let mut list = Vec::new();
        let mut i = 0;
        for _ in 0..n / 32 {
            list.push(u32::from_str_radix(&s[i..i + 32].to_string(), 2).unwrap());
            i += 32
        }

        Self {
            sign: Sign::Plus,
            value: list,
        }
    }
}

impl fmt::Display for ScInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        for v in &self.value {
            let x = format!("{:b}", v);
            let l = x.chars().collect::<Vec<char>>().len();
            result.push_str(&format!("{}{}", "0".repeat(32 - l), x));
        }

        if self.sign == Sign::Minus {
            write!(f, "-{}", result)
        } else {
            write!(f, "{}", result)
        }
    }
}
