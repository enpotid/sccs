use crate::sctype::*;
use std::{cmp::Ordering, fmt, ops};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Sign {
    Plus,
    Minus,
}

#[derive(PartialEq, Eq, Clone)]
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

        s = util::to_byte(&s);
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

impl ops::Add for ScInt {
    type Output = ScInt;

    fn add(self, other: ScInt) -> ScInt {
        &self + &other
    }
}

impl<'a> ops::Add<ScInt> for &'a ScInt {
    type Output = ScInt;

    fn add(self, other: ScInt) -> Self::Output {
        self + &other
    }
}

impl<'a> ops::Add<&'a ScInt> for ScInt {
    type Output = ScInt;

    fn add(self, other: &ScInt) -> Self::Output {
        &self + other
    }
}

impl<'a, 'b> ops::Add<&'b ScInt> for &'a ScInt {
    type Output = ScInt;

    fn add(self, other: &ScInt) -> Self::Output {
        if self.sign == other.sign {
            let a: Vec<u64> = self.value.iter().rev().map(|&s| s as u64).collect();
            let b: Vec<u64> = other.value.iter().rev().map(|&s| s as u64).collect();

            let mut result: Vec<u32> = Vec::new();
            let mut up = 0;
            for i in 0..a.len().max(b.len()) {
                let x = a.get(i).unwrap_or(&0) + b.get(i).unwrap_or(&0) + up;
                if x > u32::MAX as u64 {
                    result.push((x - u32::MAX as u64 - 1) as u32);
                    up = 1;
                } else {
                    result.push(x as u32);
                    up = 0;
                }
            }
            if up != 0 {
                result.push(up as u32);
            }

            loop {
                if let Some(x) = result.pop() {
                    if x != 0 {
                        result.push(x);
                        break;
                    }
                } else {
                    break;
                }
            }
            result.reverse();

            ScInt {
                sign: self.sign,
                value: result,
            }
        } else {
            let mut ns = self.clone();
            ns.sign = Sign::Plus;
            let mut no = other.clone();
            no.sign = Sign::Plus;
            if ns > no {
                ScInt {
                    sign: self.sign,
                    value: (ns - no).value,
                }
            } else if ns < no {
                ScInt {
                    sign: other.sign,
                    value: (no - ns).value,
                }
            } else {
                ScInt {
                    sign: Sign::Plus,
                    value: Vec::new(),
                }
            }
        }
    }
}

impl ops::Sub for ScInt {
    type Output = ScInt;

    fn sub(self, other: ScInt) -> ScInt {
        &self - &other
    }
}

impl<'a> ops::Sub<ScInt> for &'a ScInt {
    type Output = ScInt;

    fn sub(self, other: ScInt) -> Self::Output {
        self - &other
    }
}

impl<'a> ops::Sub<&'a ScInt> for ScInt {
    type Output = ScInt;

    fn sub(self, other: &ScInt) -> Self::Output {
        &self - other
    }
}

impl<'a, 'b> ops::Sub<&'b ScInt> for &'a ScInt {
    type Output = ScInt;

    fn sub(self, other: &ScInt) -> ScInt {
        if self.sign == other.sign {
            if self.sign == Sign::Plus {
                if self > other {
                    let a: Vec<i64> = self.value.iter().rev().map(|&s| s as i64).collect();
                    let b: Vec<i64> = other.value.iter().rev().map(|&s| s as i64).collect();

                    let mut result = Vec::new();
                    let mut down = 0;
                    for i in 0..a.len().max(b.len()) {
                        let x = a.get(i).unwrap_or(&0) - b.get(i).unwrap_or(&0) - down;
                        if x < 0 {
                            result.push((x + u32::MAX as i64 + 1) as u32);
                            down = 1;
                        } else {
                            result.push(x as u32);
                            down = 0;
                        }
                    }

                    loop {
                        if let Some(x) = result.pop() {
                            if x != 0 {
                                result.push(x);
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    result.reverse();

                    ScInt {
                        sign: Sign::Plus,
                        value: result,
                    }
                } else if self < other {
                    let mut n = other - self;
                    n.sign = Sign::Minus;
                    n
                } else {
                    ScInt {
                        sign: Sign::Plus,
                        value: Vec::new(),
                    }
                }
            } else {
                let mut ns = self.clone();
                ns.sign = Sign::Plus;
                let mut no = other.clone();
                no.sign = Sign::Plus;
                no - ns
            }
        } else {
            if self.sign == Sign::Plus {
                let mut no = other.clone();
                no.sign = Sign::Plus;
                self + &no
            } else {
                let mut no = other.clone();
                no.sign = Sign::Minus;
                self + &no
            }
        }
    }
}

impl Ord for ScInt {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            Ordering::Equal
        } else {
            if self.sign != other.sign {
                if self.sign == Sign::Plus {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            } else {
                let mut ns: Vec<u32> = self.value.clone().iter().rev().map(|&s| s).collect();
                loop {
                    if let Some(x) = ns.pop() {
                        if x != 0 {
                            ns.push(x);
                            break;
                        }
                    } else {
                        break;
                    }
                }
                ns.reverse();

                let mut no: Vec<u32> = other.value.clone().iter().rev().map(|&s| s).collect();
                loop {
                    if let Some(x) = no.pop() {
                        if x != 0 {
                            no.push(x);
                            break;
                        }
                    } else {
                        break;
                    }
                }
                no.reverse();

                if ns.len() > no.len() {
                    Ordering::Greater
                } else if ns.len() < no.len() {
                    Ordering::Less
                } else {
                    for i in 0..ns.len() {
                        if ns[i] > no[i] {
                            return Ordering::Greater;
                        } else if ns[i] < no[i] {
                            return Ordering::Less;
                        }
                    }
                    Ordering::Equal
                }
            }
        }
    }
}

impl PartialOrd for ScInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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

impl fmt::Debug for ScInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Sign: {}, Value: {:?}",
            if self.sign == Sign::Plus {
                "Plus"
            } else {
                "Minus"
            },
            self.value
        )
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
