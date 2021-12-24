use rayon::prelude::*;
use rug::Rational;

fn main() {
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

    println!("{:?}", result);}
