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
    pub fn from(x: &str) -> Option<Self> {
        let v: Vec<char> = x.chars().collect();
        let mut sign = Sign::Plus;

        let mut s = String::new();
        for c in v {
            if c == '-' {
                sign = Sign::Minus;
            } else if c.is_digit(10) {
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

        Some(Self { sign, value: list })
    }

    pub fn from_i128(x: i128) -> Self {
        let n = 128;
        let mut s = format!("{:b}", if x >= 0 { x } else { x * -1 });
        let l = s.chars().collect::<Vec<char>>().len();
        s = format!("{}{}", "0".repeat(n - l), s);
        let mut list = Vec::new();
        let mut i = 0;
        for _ in 0..n / 32 {
            list.push(u32::from_str_radix(&s[i..i + 32].to_string(), 2).unwrap());
            i += 32
        }

        Self {
            sign: if x >= 0 { Sign::Plus } else { Sign::Minus },
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
            write!(f, "-{}", crate::sctype::util::byte_to_num(&result))
        } else {
            write!(f, "{}", crate::sctype::util::byte_to_num(&result))
        }
    }
}

impl fmt::Binary for ScInt {
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
