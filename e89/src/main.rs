use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn as_minimal_roman(n: u32) -> String {
    let mut n = n;
    let mut s = String::new();

    while n >= 1000 {
        s.push_str("M");
        n -= 1000;
    }
    if n >= 900 {
        s.push_str("CM");
        n -= 900;
    }
    if n >= 500 {
        s.push_str("D");
        n -= 500;
    }
    if n >= 400 {
        s.push_str("CD");
        n -= 400;
    }
    while n >= 100 {
        s.push_str("C");
        n -= 100;
    }
    if n >= 90 {
        s.push_str("XC");
        n -= 90;
    }
    if n >= 50 {
        s.push_str("L");
        n -= 50;
    }
    if n >= 40 {
        s.push_str("XL");
        n -= 40;
    }
    while n >= 10 {
        s.push_str("X");
        n -= 10;
    }
    if n == 9 {
        s.push_str("IX");
        n -= 9;
    }
    if n >= 5 {
        s.push_str("V");
        n -= 5;
    }
    if n == 4 {
        s.push_str("IV");
        n -= 4;
    }
    while n > 0 {
        s.push_str("I");
        n -= 1;
    }
    s
}

fn parse_as_roman(s: &str) -> Result<u32, String> {
    if s.len() == 0 {
        return Err("empty input string".to_string());
    }
    let mut result = 0u32;
    let mut skip = false;
    let s_bytes = s.as_bytes();
    for (i, &c) in s_bytes.iter().enumerate() {
        if skip {
            skip = false;
            continue;
        }
        if c == b'M' {
            result += 1000;
        } else if c == b'C' {
            if i + 1 < s_bytes.len() && s_bytes[i + 1] == b'M' {
                result += 900;
                skip = true;
            } else if i + 1 < s_bytes.len() && s_bytes[i + 1] == b'D' {
                result += 400;
                skip = true;
            } else {
                result += 100;
            }
        } else if c == b'D' {
            result += 500;
        } else if c == b'L' {
            result += 50;
        } else if c == b'X' {
            if i + 1 < s_bytes.len() && s_bytes[i + 1] == b'C' {
                result += 90;
                skip = true;
            } else if i + 1 < s_bytes.len() && s_bytes[i + 1] == b'L' {
                result += 40;
                skip = true;
            } else {
                result += 10;
            }
        } else if c == b'V' {
            result += 5;
        } else if c == b'I' {
            if i + 1 < s_bytes.len() && s_bytes[i + 1] == b'V' {
                result += 4;
                skip = true;
            } else if i + 1 < s_bytes.len() && s_bytes[i + 1] == b'X' {
                result += 9;
                skip = true;
            } else {
                result += 1;
            }
        } else {
            return Err(format!("invalid input: {}", s).to_string());
        }
    }
    Ok(result)
}

fn read_numbers<P>(filename: P) -> Vec<(String, u32)>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    BufReader::new(file)
        .lines()
        .map(|l| {
            let original = l.unwrap();
            (original.clone(), parse_as_roman(&original).unwrap())
        })
        .collect()
}

fn main() {
    let numbers = read_numbers("p089_roman.txt");
    let v: Vec<u32> = numbers
        .iter()
        .map(|(original, n)| (original, as_minimal_roman(*n)))
        .inspect(|(o, m)| {
            println!(
                "original: {}\tminimal: {}\tdiff: {}",
                o,
                m,
                o.len() - m.len()
            )
        })
        .map(|(original, minimal)| original.len() as u32 - minimal.len() as u32)
        .collect::<Vec<u32>>();
    println!("n: {}", v.iter().sum::<u32>());
}

mod test {

    use super::*;

    #[test]
    fn test_to_roman() {
        assert_eq!(as_minimal_roman(4), "IV");
        assert_eq!(as_minimal_roman(14), "XIV");
        assert_eq!(as_minimal_roman(10), "X");
        assert_eq!(as_minimal_roman(16), "XVI");
        assert_eq!(as_minimal_roman(19), "XIX");
        assert_eq!(as_minimal_roman(49), "XLIX");
    }

    #[test]
    fn test_from_roman() {
        assert_eq!(parse_as_roman("IV").unwrap(), 4);
        assert_eq!(parse_as_roman("XIV").unwrap(), 14);
        assert_eq!(parse_as_roman("X").unwrap(), 10);
        assert_eq!(parse_as_roman("XVI").unwrap(), 16);
        assert_eq!(parse_as_roman("XIX").unwrap(), 19);
        assert_eq!(parse_as_roman("XLIX").unwrap(), 49);
    }
}