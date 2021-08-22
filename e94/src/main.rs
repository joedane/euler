use num_rational::Ratio;
use rayon::prelude::*;

fn isqrt(n: u128) -> u128 {
    let mut x0 = n / 2;

    if x0 != 0 {
        let mut x1 = (x0 + n / x0) >> 1;
        while x1 < x0 {
            x0 = x1;
            x1 = (x0 + n / x0) >> 1;
        }
        return x0;
    } else {
        return n;
    }
}

fn is_e94(a: u32, b: u32, c: u32) -> Option<bool> {
    //    println!("a:{}, b:{}, c:{}", a, b, c);
    let p = a + b + c;
    if p > 1000000000 {
        return None;
    }
    let (a_f, b_f, c_f) = (
        Ratio::from_integer(a as u128),
        Ratio::from_integer(b as u128),
        Ratio::from_integer(c as u128),
    );

    let s = (a_f + b_f + c_f) / Ratio::from_integer(2 as u128);
    //    println!("s: {}, a: {}, b: {}, c:{}", s, a_f, b_f, c_f);
    let area_sq = s * (s - a_f) * (s - b_f) * (s - c_f);

    if area_sq.is_integer() {
        let a_sq_i = area_sq.to_integer();
        let area = isqrt(a_sq_i);
        if area * area == a_sq_i {
            Some(true)
        } else {
            Some(false)
        }
    } else {
        Some(false)
    }
}

struct TripleIter {
    i: u32,
    index: usize,
}

impl Iterator for TripleIter {
    type Item = (u32, u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == 0 {
            self.index = 1;
            Some((self.i, self.i, self.i))
        } else if self.index == 1 {
            self.index = 2;
            Some((self.i, self.i, self.i + 1))
        } else if self.index == 2 {
            self.index = 3;
            Some((self.i, self.i + 1, self.i + 1))
        } else {
            None
        }
    }
}

fn main() {
    let triples: Vec<(u32, u32, u32)> = (2u32..1000000005)
        .into_par_iter()
        .flat_map_iter(|i| TripleIter { i: i, index: 0 })
        .filter_map(|triple| {
            if let Some(b) = is_e94(triple.0, triple.1, triple.2) {
                if b {
                    Some(triple)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();
    println!("{:?}", triples);
    println!(
        "total perimeters: {}",
        triples.iter().fold(0, |acc, x| acc + x.0 + x.1 + x.2)
    );

    /*
    for i in 2.. {
        if i % 100000 == 0 {
            println!("{}", i);
        }
        if let Some(b) = is_e94(i, i, i) {
            if b {
                println!("[{}, {}, {}", i, i, i);
                count += 1;
                total_p += (i + i + i) as u64;
            }
        } else {
            break;
        }

        if let Some(b) = is_e94(i, i, i + 1) {
            if b {
                println!("[{}, {}, {}", i, i, i + 1);
                count += 1;
                total_p += (i + i + i + 1) as u64;
            }
        } else {
            break;
        }

        if let Some(b) = is_e94(i, i + 1, i + 1) {
            if b {
                println!("[{}, {}, {}", i, i + 1, i + 1);
                count += 1;
                total_p += (i + i + i + 2) as u64;
            }
        } else {
            break;
        }
    }
    println!("Count: {}", count);
    println!("Total perimiter: {}", total_p);
    */
}

mod test {

    use super::*;

    #[test]
    fn test1() {
        assert_eq!(4, isqrt(16));
        assert_eq!(4, isqrt(17));
        assert_eq!(10, isqrt(100));
        assert_eq!(10, isqrt(101));
    }
}
