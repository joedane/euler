use std::collections::HashMap;
use std::collections::HashSet;
use rayon::prelude::*;

use euler_rust::*;


/*
fn divisors(n:u64, cache:&mut HashMap<u64, Vec<u64>>) -> Vec<u64> {
    if cache.contains_key(&n) {
        return cache.get(&n).unwrap().to_vec();
    }
        
    let mut d = euler_rust::divisors(n).collect::<Vec<_>>();

    /*
    while d*2 <= n {
        if n % d == 0 {
            divisors.push(d);
        }
        d +=1;
    }
     */
    cache.insert(n, d.clone());
//    println!("divisors of {}: {:?}", n, d);
    d
}
 */

fn factor(n: u64, primes: &[u64]) -> HashMap<u64, u8> {
    let mut factors = HashMap::new();
    let mut tmp = n;
//    println!("factoring {}", n);
    for factor in primes {
//        println!("testing {}", factor);
        if factor * 2 > n {
//            println!("pau");
            break;
        }
        while tmp % factor == 0 {
//            println!("{} is a factor", factor);
            let entry = factors.entry(*factor).or_insert(0u8);
            *entry += 1;
            tmp /= factor;
        }
    }
    factors
}

fn divisor_sum(n: u32, primes: &[u64]) -> u32 {
    //divisors(n, cache).iter().fold(0, |acc, &x| acc + x)
    let factors = factor(n.into(), primes);
    let mut result:u32 = 1;
//    println!("divisor_sum: {}", n);
    if factors.len() == 0 {
        return 1;  //  "proper" divisors include 1 but omit n
    }
    for (p, e) in factors.iter() {
//        println!("p: {}\te:{}", p, e);
        let step = ((p.pow((e+1).into()) - 1) / (p - 1)) as u32;
//        println!("step: {}", step);
        result = result * step;
//        println!("result: {}", result);
    }
    (result - n) as u32
}

fn vec_contains(v:&Vec<u32>, n:u32) -> bool {
    for &i in v {
        if i == n {
            return true;
        }
    }
    return false;
}

fn trim_chain(v:&Vec<u32>, n:u32) -> Vec<u32> {
    let start = v.iter().position(|&i| i == n).unwrap();
    v[start..v.len()].to_vec()
}

fn main() {
    const MAX:u32 = 1000000;
    let mut seen:HashSet<u32> = HashSet::new();
    let mut longest_chain:Vec<u32> = Vec::new();;
    let mut this_chain:Vec<u32> = Vec::new();
//    let mut divisor_cache = HashMap::new();
    let primes = euler_rust::primes().take_while(|x| *x as u32 <= MAX/2).collect::<Vec<u64>>();
    //let mut ar = [0u32;MAX as usize];
    
    
    let ar:HashMap<u32, u32> = (2..MAX).into_par_iter()
        .map(|x| (x, divisor_sum(x, &primes)))
        .collect();
    
    'outer: for n in (2..MAX) {
        if seen.contains(&n) {
            continue;
        }
        let mut this = n;
        loop {
            this_chain.push(this);
            seen.insert(this);
            match ar.get(&this) {
                Some(&next) => {
                    if next > MAX {
                        this_chain.clear();
                        continue 'outer;
                    } else if vec_contains(&this_chain, next) {
                        this_chain = trim_chain(&this_chain, next);
                        if this_chain.len() > longest_chain.len() {
                            longest_chain = this_chain.clone();
                        }
                        this_chain.clear();
                        continue 'outer;
                    } else {
                        this = next;
                    }
                },
                None => {
                    //println!("no successor for {}", this);
                    this_chain.clear();
                    continue 'outer;
                }
            }
        }
    }
    println!("{:?}", longest_chain);

}

#[cfg(test)]
mod tests {

    use super::*;

    /*
    #[test]
    fn test_div1() {
        assert_eq!(divisors(7), vec![1]);
        assert_eq!(divisors(12), vec![1, 2, 3, 4, 6]);
        assert_eq!(divisors(28), vec![1, 2, 4, 7, 14]);
        assert_eq!(divisors(25), vec![1, 5]);
    }
     */

    /*
    #[test]
    fn test_perfect1() {
        assert!(is_perfect(28));
        assert!(!is_perfect(12));
    }
     */
    
    #[test]
    fn test_150() {
        //        println!("150: {:?}", divisors(150));
        println!("72 sum: {:?}", divisor_sum(72));
        println!("28 sum: {:?}", divisor_sum(28));
        println!("220 sum: {:?}", divisor_sum(220));
        println!("284 sum: {:?}", divisor_sum(284));
        println!("150 sum: {:?}", divisor_sum(150));
        println!("222 sum: {:?}", divisor_sum(222));
        println!("222 sum: {:?}", divisor_sum(234));
        println!("312 sum: {:?}", divisor_sum(312));
    }

}
        
