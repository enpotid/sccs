use std::{fmt, ops};

pub struct ScUInt {
    pub value: Vec<u32>,
}

impl ScUInt {
    pub fn from(x: &str) -> Option<Self> {
        let v: Vec<char> = x.chars().collect();
        let mut s = String::new();
        for c in v {
            if c.is_digit(10) {
                s.push(c);
            } else {
                return None;
            }
        }

        s = crate::sctype::util::to_byte(&s);
        let l = s.chars().collect::<Vec<char>>().len();
        let n = if l % 32 != 0 { l + 32 - (l % 32) } else { l };
        s = format!("{}{}", "0".repeat(n - l), s);
        let mut list = Vec::new();
        let mut i = 0;
        for _ in 0..n / 32 {
            list.push(u32::from_str_radix(&s[i..i + 32].to_string(), 2).unwrap());
            i += 32
        }

        Some(Self { value: list })
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

        Self { value: list }
    }
}

impl ops::Add for ScUInt {
    type Output = ScUInt;

    fn add(self, other: ScUInt) -> ScUInt {
        let a: Vec<u64> = self.value.iter().rev().map(|&s| s as u64).collect();
        let b: Vec<u64> = other.value.iter().rev().map(|&s| s as u64).collect();

        let mut result = Vec::new();
        let mut up = 0;
        for i in 0..a.len().max(b.len()) {
            let x = a.get(i).unwrap_or(&0) + b.get(i).unwrap_or(&0) + up;
            if x > u32::MAX as u64 {
                let b = format!("{:b}", x);
                let l = b.len();
                let u = b[0..l - 32].to_string();
                let h = b[l - 32..].to_string();

                result.push(u32::from_str_radix(&h, 2).unwrap());
                up = u32::from_str_radix(&u, 2).unwrap() as u64;
            } else {
                result.push(x as u32);
                up = 0;
            }
        }
        if up != 0 {
            result.push(up as u32);
        }

        result.reverse();
        ScUInt { value: result }
    }
}

impl fmt::Display for ScUInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        for v in &self.value {
            let x = format!("{:b}", v);
            let l = x.chars().collect::<Vec<char>>().len();
            result.push_str(&format!("{}{}", "0".repeat(32 - l), x));
        }

        write!(f, "{}", crate::sctype::util::byte_to_num(&result))
    }
}

impl fmt::Binary for ScUInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        for v in &self.value {
            let x = format!("{:b}", v);
            let l = x.chars().collect::<Vec<char>>().len();
            result.push_str(&format!("{}{}", "0".repeat(32 - l), x));
        }

        write!(f, "{}", result)
    }
}
