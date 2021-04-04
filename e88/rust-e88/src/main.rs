use std::collections::HashSet;
use std::collections::HashMap;


fn first_factor(n:u32) -> u32 {

    if n % 2 == 0 {
        return 2;
    }
    for f in (1..).map(|m| 2*m + 1).take_while(|m| m*m <= n) {
        if n % f == 0 {
            return f;
        }
    }
    return n;
}

/**
 * return a HashMap factorization of n, with prime factors as keys
 */
fn factor(n: u32) -> HashMap<u32, u32> {
    let mut factors = HashMap::new();
    let mut cur = n;
    loop {
        let f = first_factor(cur);
        if f == n {
            break;
        }
        let this_factor = factors.entry(f).or_insert(0);
        *this_factor += 1;
        if cur == f {
            break;
        }
        cur /= f;
    }
    factors
}

fn is_this_product_sum(factors: &Vec<u32>, k:usize) -> bool {
    if factors.len() > k {
        panic!("too many factors ({}) than k {}", factors.len(), k);
    }
    factors.iter().fold(1, |acc, x| acc * x) == factors.iter().fold(0, |acc, x| acc + x) + (k-factors.len()) as u32
}

fn is_product_sum_step(factors: &mut Vec<u32>, times: u32, k:usize) -> Option<Vec<u32>> {
    for i in 0..factors.len() {
        let saved = factors[i];
        factors[i] *= times;
        if is_this_product_sum(factors, k) {
            return Some(factors.clone());
        }
        factors[i] = saved;
    }
    return None;
}

fn is_product_sum_1(factors: Vec<u32>, k:usize) -> Option<Vec<u32>> {
    for i in 0..factors.len() {
        let (left, right) = factors.split_at(i);
        let this_factor = factors[i];
        let mut subfactors = [left, &right[1..]].concat();
        if let Some(f) = is_product_sum_step(&mut subfactors, this_factor, k) {
            return Some(f);
        }
        if let Some(f) = is_product_sum_1(subfactors, k) {
            return Some(f);
        }
    }
    None
}

fn is_product_sumX(n: u32, k:usize) -> Option<Vec<u32>> {
    let factors = factor(n);
    if factors.len() > k {
        panic!("number {} has more factors ({}) than k {}", n, factors.len(), k);
    }
    if is_this_product_sum(&factors, k) {
        return Some(factors);
    }
    is_product_sum_1(factors, k)
}

fn product_equals_p(v: &Vec<i32>, n:i32) -> bool {
    let mut sum = 1i32;
    
    for i in 0..v.len()-1 {
        sum = sum * v[i];
        if sum > n {
            return false;
        }
    }
    if sum == n {
        return true;
    } else {
        return false;
    }
}
                    
fn is_product_sum(n: i32, k:usize) -> Option<Vec<i32>> {
    // Knuth "Algorithm H" from Fasicle 3b
    let mut a = vec![1i32; k+1];
    a[0] = n as i32 - k as i32 + 1;
    a[k] = -1;
    let mut tries = 1u32;
    
    loop {
        println!("try {}", tries);
        tries = tries + 1;
        if product_equals_p(&a, n) {
            return Some(a);
        }
        if a[1] < a[0] - 1 {
            a[0] = a[0] - 1;
            a[1] = a[1] + 1;
            continue;
        }
        let mut j = 2;
        let mut s = a[0] + a[1] - 1;
        loop {
            if a[j] < a[0] - 1 {
                break;
            }
            s = s + a[j];
            j = j + 1;
        }
        if j + 1 > k {
            return None;
        }
        let x = a[j] + 1;
        a[j] = x;
        j = j - 1;
        loop {
            if j == 0 {
                break;
            }
            a[j] = x;
            s = s - x;
            j = j - 1;
        }
        a[0] = s;
    }
}



fn min_product_sum(k: usize) -> (i32, Vec<i32>) {
    for n in k as i32.. {
        println!("\tn: {}", n);
        if let Some(f) = is_product_sum(n, k) {
            return (n, f);
        }
    }
    unreachable!();
}

fn main() {

    println!(factor(12));
    /*
    let mut mins = HashSet::new();
    
    for k in 2..12001 {
        let (n, factors) = min_product_sum(k);
    //    println!("k: {}, n: {}, factors: {:?}", k, n, factors);
        mins.insert(n);
        println!("k: {}", k);
    }
    println!("mins: {:?}", mins);
    println!("sum: {}", mins.iter().fold(0, |acc, x| acc + x));
    */
}

mod test {

    use super::*;

    #[test]
    fn test_factor1() {
        let mut l = factor(12);
        assert_eq!(l, vec![2, 2, 3]);

        l = factor(25);
        assert_eq!(l, vec![5, 5]);

        l = factor(13);
        assert_eq!(l, vec![]);
    }

    #[test]
    fn test_isPS1() {
        assert!(is_this_product_sum(&vec![2, 2, 2], 5));
        assert!(!is_this_product_sum(&vec![2, 2, 3], 5));
    }

    #[test]
    fn test_isPS3() {
        is_product_sum(12, 6).unwrap();
    }

}


