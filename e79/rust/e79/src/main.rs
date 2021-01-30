use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};

struct Test {
    parts: [u8; 3]
}

impl Test {
   
    fn test(&self, code: [u8;10]) -> bool {
        if let Some(i) = index_of(self.parts[0], code, 0) {
            if let Some(j) = index_of(self.parts[1], code, i+1) {
                if let Some(_) = index_of(self.parts[2], code, j+1) {
                    return true;
                }
            }
        }
        return false;
    }
}

fn index_of(n: u8, code: [u8;10], start: usize) -> Option<usize> {
    for i in start..10 {
        if n == code[i] {
            return Some(i);
        }
    }
    return None;
}


fn make_test(s: String) -> Test {
    assert!(s.len() == 3);
    let mut p: [u8;3] = [0, 0, 0];
    p[0] = s[0..1].parse().unwrap();
    p[1] = s[1..2].parse().unwrap();
    p[2] = s[2..3].parse().unwrap();
    Test { parts: p }
}

fn read_tests<P>(filename: P) -> Vec<Test>
where P: AsRef<Path> {
    let file = File::open(filename).unwrap();
    BufReader::new(file).lines()
        .map(|l| make_test(l.unwrap()))
        .collect()
}

fn increment_code(code: &mut [u8;10]) {
    let mut i = 9;
    loop {
        if code[i] < 9 {
            code[i] += 1;
            break;
        }
        code[i] = 0;
        if i == 0 {
            break;
        }
        i -= 1;
    }
}


fn main() {
    let tests = read_tests("./p079_keylog.txt");
    let mut code: [u8;10] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ];

    loop {
        if tests.iter().all(|t| t.test(code)) {
            println!("{:?}", code);
            break;
        }
        increment_code(&mut code);
    }
    
    println!("Hello, world!");
}

#[cfg(test)]

mod tests {

    use crate::*;

    #[test]
    fn test_incr1() {
        let mut code = [0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
        increment_code(&mut code);
        assert_eq!(code, [0, 0, 0, 0, 0, 0, 0, 0, 0, 2]);
    }

    #[test]
    fn test_incr2() {
        let mut code = [0, 0, 0, 0, 0, 0, 0, 0, 0, 9];
        increment_code(&mut code);
        assert_eq!(code, [0, 0, 0, 0, 0, 0, 0, 0, 1, 0]);
    }

    #[test]
    fn test_test1() {
        let test = Test { parts: [1, 2, 3]};
        let code = [0, 0, 0, 0, 0, 0, 0, 1, 2, 3];
        assert!(test.test(code));
    }

        #[test]
    fn test_test2() {
        let test = Test { parts: [1, 2, 3]};
        let code = [0, 0, 1, 0, 2, 0, 0, 1, 2, 3];
        assert!(test.test(code));
    }

    
}
