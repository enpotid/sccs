pub fn to_byte(x: &str) -> String {
    let mut nx = x.to_string();
    let mut binary_str = String::new();

    if nx == "0" {
        return "0".to_string();
    }

    while nx != "0" {
        let remainder = modt(&nx);
        binary_str.push_str(&remainder);
        nx = divt(&nx);
    }

    binary_str.chars().rev().collect()
}

pub fn byte_to_num(x: &str) -> String {
    let cs: Vec<char> = x.chars().rev().collect();
    let mut p = String::from("1");
    let mut result = String::from("0");
    for c in cs {
        if c == '1' {
            result = plss(&result, &p);
        }
        p = plss(&p, &p);
    }
    result
}

fn divt(x: &str) -> String {
    let cs: Vec<usize> = x.chars().map(|s| s as usize - '0' as usize).collect();
    let mut result = String::from("0");
    let mut i = 0;
    for c in cs.iter().rev() {
        result = plss(&result, &divti(*c, i));
        i += 1;
    }
    result
}

fn divti(x: usize, i: usize) -> String {
    if x == 0 {
        return String::from("0");
    }

    if i == 0 {
        return (x / 2).to_string();
    }

    return format!(
        "{}{}{}",
        x / 2,
        if x % 2 == 0 { 0 } else { 5 },
        "0".repeat(i - 1)
    );
}

fn plss(a: &str, b: &str) -> String {
    let ac: Vec<u8> = a.chars().map(|s| s as u8 - '0' as u8).rev().collect();
    let bc: Vec<u8> = b.chars().map(|s| s as u8 - '0' as u8).rev().collect();

    let mut result: Vec<char> = Vec::new();
    let mut up = 0;
    for i in 0..ac.len().max(bc.len()) {
        let x = ac.get(i).unwrap_or(&0) + bc.get(i).unwrap_or(&0) + up;
        up = x / 10;
        result.push((x % 10 + '0' as u8) as char)
    }
    if up != 0 {
        result.push((up + '0' as u8) as char);
    }
    result.iter().rev().collect::<String>()
}

fn modt(x: &str) -> String {
    ((x.chars().last().unwrap() as u8 - '0' as u8) % 2).to_string()
}
