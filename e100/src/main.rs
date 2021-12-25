
//use rayon::prelude::*;
//use rug::Rational;

struct Sqrt2Convergents {
    a_1: u64,
    a_2: u64,
    b_1: u64,
    b_2: u64
}

impl Sqrt2Convergents {

    fn new() -> Self {
        Sqrt2Convergents {
            a_1: 1,
            a_2: 1,
            b_1: 1,
            b_2: 0
        }
    }
}

impl Iterator for Sqrt2Convergents {

    type Item = (u64, u64);

    fn next(&mut self) -> Option<Self::Item> {
        let val = (2*self.a_1 + self.a_2, 2*self.b_1 + self.b_2);
        self.a_2 = self.a_1;
        self.a_1 = val.0;
        self.b_2 = self.b_1;
        self.b_1 = val.1;
        Some(val)
    }
}

fn main() {

    let s = Sqrt2Convergents::new();

    let p = 15u64;
    let q = 21u64;

    let results = s.map(|(r, s)|(p*r + 2*q*s, p*s + q*r)).take(10).collect::<Vec<(u64, u64)>>();

    println!("{:?}", results);


    /*
    let target = Rational::from((1, 2));

    let result = (1_000_000u64..2_000_000)
        .into_par_iter()
        .find_first(|&n| {
            let mut b = 1;
            let mut b_low = 1;
            let mut b_high = n;
            println!("checking n = {}", n);
            loop {
                // println!("try b = {}, b_low = {}, b_high = {}", b, b_low, b_high);
                let r = Rational::from((b, n)) * Rational::from((b - 1, n - 1));
                if r == target {
                    println!("b = {}, n = {}", b, n);
                    return true;
                } else if b_high == b_low + 1 {
                    println!("not found [n = {}, b = {}, b_low = {}, b_high = {}, r = {}]", n, b, b_low, b_high, r);
                    return false;
                } else if r < target {
                    b_low = b;
                    b = (b + b_high) / 2;
                } else {
                    b_high = b;
                    b = (b + b_low) / 2;
                }
            }
        });

    println!("{:?}", result);
*/
    }